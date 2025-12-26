use rustc_ast::{self as ast, Attribute, NodeId, Item};
use rustc_data_structures::fx::{FxHashMap, FxIndexMap};
use rustc_errors::{DiagCtxtHandle, ErrorGuaranteed};
use rustc_feature::Features;
use rustc_ast::attr::MarkedAttrs;
use rustc_hir::limit::Limit;
use rustc_lint_defs::RegisteredTools;
use rustc_session::Session;
use rustc_span::LocalExpnId;
use rustc_span::def_id::{DefId, LocalDefId};
use crate::prelude::*;
use rustc_span::hygiene::{MacroKind};
use rustc_span::{Span, Symbol, DUMMY_SP, kw, sym};
use rustc_parse::parser::{Parser};
use rustc_parse::MACRO_ARGUMENTS;
use rustc_session::parse::ParseSess;
use rustc_span::source_map::SourceMap;
use rustc_errors::PResult;
use rustc_ast::tokenstream::TokenStream;
use rustc_errors::BufferedEarlyLint;

use crate::resolver_traits::OpaqueDeriveResolution;
use crate::annotatable::Annotatable;
use crate::syntax_extension_trait::SyntaxExtensionTrait;
use crate::invocation_data::{ExpansionData, DirOwnership};
use crate::struct_macrostat::MacroStat;
use crate::base_expansion_context::ResolverExpand;
use crate::base_expansion_context::BaseExpansionContext;
use rustc_session::config::CollapseMacroDebuginfo;
use rustc_hir::{Stability, find_attr};
use rustc_ast::visit::{AssocCtxt, Visitor};
use rustc_ast::{AttrVec, HasAttrs, PatKind, Safety};
use rustc_data_structures::sync;
use rustc_hir as hir;
use rustc_hir::attrs::{AttributeKind, CfgEntry, Deprecation};
use rustc_expand_meta_macros::{ExpanderStruct, CtxSpecial};

pub trait LintStoreExpand {
    fn pre_expansion_lint(
        &self,
        sess: &Session,
        features: &Features,
        registered_tools: &RegisteredTools,
        node_id: NodeId,
        attrs: &[Attribute],
        items: &[Box<Item>],
        name: Symbol,
    );
}

pub type LintStoreExpandDyn<'a> = Option<&'a (dyn LintStoreExpand + 'a)>;

#[macro_export]
macro_rules! ExtCtxt_Def {
    ($DRT:ident) => {
        pub struct ExtCtxt<'a, $DRT: OpaqueDeriveResolution + 'static> {
            pub sess: &'a Session,
            pub ecfg: ExpansionConfig<'a>,
            pub num_standard_library_imports: usize,
            pub reduced_recursion_limit: Option<(Limit, ErrorGuaranteed)>,
            pub root_path: PathBuf,
            pub resolver: &'a mut (dyn ResolverExpand<$DRT> + 'static),
            pub current_expansion: ExpansionData,
            /// Error recovery mode entered when expansion is stuck
            /// (or during eager expansion, but that's a hack).
            pub force_mode: bool,
            pub expansions: FxIndexMap<Span, Vec<String>>,
            /// Used for running pre-expansion lints on freshly loaded modules.
            pub(super) lint_store: LintStoreExpandDyn<'a>,
            /// Used for storing lints generated during expansion, like `NAMED_ARGUMENTS_USED_POSITIONALLY`
            pub buffered_early_lint: Vec<BufferedEarlyLint>,
            /// When we 'expand' an inert attribute, we leave it
            /// in the AST, but insert it here so that we know
            /// not to expand it again.
            pub(super) expanded_inert_attrs: MarkedAttrs,
            /// `-Zmacro-stats` data.
            pub macro_stats: FxHashMap<(Symbol, MacroKind), MacroStat>,
            pub nb_macro_errors: usize,
        }

        impl<'a, $DRT: OpaqueDeriveResolution + 'static> ExtCtxt<'a, $DRT> {
            pub fn new(
                sess: &'a Session,
                ecfg: ExpansionConfig<'a>,
                resolver: &'a mut (dyn ResolverExpand<$DRT> + 'static),
                lint_store: LintStoreExpandDyn<'a>,
            ) -> ExtCtxt<'a, $DRT> {
                ExtCtxt {
                    sess,
                    ecfg,
                    num_standard_library_imports: 0,
                    reduced_recursion_limit: None,
                    resolver,
                    lint_store,
                    root_path: PathBuf::new(),
                    current_expansion: ExpansionData {
                        id: LocalExpnId::ROOT,
                        depth: 0,
                        module: Default::default(),
                        dir_ownership: DirOwnership::Owned { relative: None },
                        lint_node_id: ast::CRATE_NODE_ID,
                        is_trailing_mac: false,
                    },
                    force_mode: false,
                    expansions: FxIndexMap::default(),
                    expanded_inert_attrs: MarkedAttrs::new(),
                    buffered_early_lint: vec![],
                    macro_stats: Default::default(),
                    nb_macro_errors: 0,
                }
            }

            pub fn dcx(&self) -> DiagCtxtHandle<'a> {
                self.sess.dcx()
            }


            pub fn new_parser_from_tts(&self, stream: TokenStream) -> Parser<'a> {
                Parser::new(&self.sess.psess, stream, MACRO_ARGUMENTS)
            }
            pub fn source_map(&self) -> &'a SourceMap {
                self.sess.psess.source_map()
            }
            pub fn psess(&self) -> &'a ParseSess {
                &self.sess.psess
            }
            pub fn call_site(&self) -> Span {
                self.current_expansion.id.expn_data().call_site
            }

            /// Returns the current expansion kind's description.
            pub fn expansion_descr(&self) -> String {
                let expn_data = self.current_expansion.id.expn_data();
                expn_data.kind.descr()
            }

            /// Equivalent of `Span::def_site` from the proc macro API,
            /// except that the location is taken from the span passed as an argument.
            pub fn with_def_site_ctxt(&self, span: Span) -> Span {
                span.with_def_site_ctxt(self.current_expansion.id.to_expn_id())
            }

            /// Equivalent of `Span::call_site` from the proc macro API,
            /// except that the location is taken from the span passed as an argument.
            pub fn with_call_site_ctxt(&self, span: Span) -> Span {
                span.with_call_site_ctxt(self.current_expansion.id.to_expn_id())
            }

            /// Equivalent of `Span::mixed_site` from the proc macro API,
            /// except that the location is taken from the span passed as an argument.
            pub fn with_mixed_site_ctxt(&self, span: Span) -> Span {
                span.with_mixed_site_ctxt(self.current_expansion.id.to_expn_id())
            }

            /// Returns span for the macro which originally caused the current expansion to happen.
            ///
            /// Stops backtracing at include! boundary.
            pub fn expansion_cause(&self) -> Option<Span> {
                self.current_expansion.id.expansion_cause()
            }

            /// This method increases the internal macro errors count and then call `trace_macros_diag`.
            pub fn macro_error_and_trace_macros_diag(&mut self) {
                self.nb_macro_errors += 1;
                self.trace_macros_diag();
            }

            pub fn trace_macros_diag(&mut self) {
                for (span, notes) in self.expansions.iter() {
                    let mut db = self.dcx().create_note(rustc_expand_base_lib_errors::TraceMacroBase { span: *span });
                    for note in notes {
                        #[allow(rustc::untranslatable_diagnostic)]
                        db.subdiagnostic(rustc_expand_base_lib_errors::TraceMacroNote { span: *span, message: note.clone() });
                    }
                    db.emit();
                }
                // Fixme: does this result in errors?
                self.expansions.clear();
            }
            pub fn trace_macros(&self) -> bool {
                self.ecfg.trace_mac
            }
            pub fn set_trace_macros(&mut self, x: bool) {
                self.ecfg.trace_mac = x
            }
            pub fn std_path(&self, components: &[Symbol]) -> Vec<Ident> {
                let def_site = self.with_def_site_ctxt(DUMMY_SP);
                iter::once(Ident::new(kw::DollarCrate, def_site))
                    .chain(components.iter().map(|&s| Ident::with_dummy_span(s)))
                    .collect()
            }
            pub fn def_site_path(&self, components: &[Symbol]) -> Vec<Ident> {
                let def_site = self.with_def_site_ctxt(DUMMY_SP);
                components.iter().map(|&s| Ident::new(s, def_site)).collect()
            }


        }
    };
}
