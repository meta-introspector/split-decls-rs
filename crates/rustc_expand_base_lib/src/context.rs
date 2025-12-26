use std::any::Any;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::default::Default;
use std::iter;

use rustc_ast::{self as ast, Attribute, NodeId, Item};
use rustc_span::Ident;
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
//use crate::config::ExpansionConfig;
//use crate::errors;

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
use crate::ExtCtxt_Def;
ExtCtxt_Def!(DRT);

#[macro_export]
macro_rules! define_expansion_context_types {
    ($DRT:ident) => {

        DeriveResolutionGeneric!($DRT) {
            pub path: ast::Path,
            pub item: Annotatable,
            // FIXME: currently this field is only used in `is_none`/`is_some` conditions. However, the
            // `Arc<SyntaxExtension>` will be used if the FIXME in `MacroExpander::fully_expand_fragment`
            // is completed.
            pub exts: crate::t_define_expansion_context_types_exts::TDefineExpansionContextTypesExts!($DRT),
            pub is_const: bool,
        }



        pub type ExpandContext<'b, DRT> = ExtCtxtGeneric!('b', DRT);
        pub type RefAMutExpandContext_B_DRT<'a, 'b, DRT> = &'a mut ExpandContext<'b, DRT>;
        ExpanderStruct!();

        // Implement the BaseExpansionContext trait for ExtCtxt
        impl<'a, DRT: OpaqueDeriveResolution + 'static> BaseExpansionContext<DRT> for ExtCtxt<'a, DRT> {
            fn dcx(&self) -> DiagCtxtHandle {
                self.sess.dcx()
            }

            fn current_expansion(&self) -> &ExpansionData {
                &self.current_expansion
            }

            fn current_expansion_mut(&mut self) -> &mut ExpansionData {
                &mut self.current_expansion
            }

            fn ecfg(&self) -> &ExpansionConfig {
                // Since ecfg is already of type crate::expand::ExpansionConfig<'a> (which is now
                // rustc_expand_base_lib::ExpansionConfig<'a>), this direct reference is fine.
                &self.ecfg
            }

            fn resolver(&mut self) -> &mut (dyn ResolverExpand<DRT> + 'static) {
                self.resolver
            }

            fn features(&self) -> &Features {
                self.ecfg.features
            }

            fn emit_recursion_limit_reached(&mut self, span: Span, descr: &str, suggested_limit: Limit, crate_name: Symbol) -> ErrorGuaranteed {
                let guar = self.dcx().emit_err(format!(
                    "recursion limit reached while expanding `{}` in `{}`",
                    descr, crate_name
                ));
                self.macro_error_and_trace_macros_diag();
                guar
            }

            fn emit_wrong_fragment_kind(&mut self, span: Span, kind: &str, name_path: &ast::Path) -> ErrorGuaranteed {
                let guar = self.dcx().emit_err(format!(
                    "macro can only expand to {} fragments, not `{}`",
                    kind,
                    name_path.segments[0].ident
                ));
                self.macro_error_and_trace_macros_diag();
                guar
            }

            fn macro_error_and_trace_macros_diag(&mut self) {
                self.nb_macro_errors += 1;
                self.trace_macros_diag();
            }

            fn next_node_id(&mut self) -> NodeId {
                self.resolver.next_node_id()
            }

            fn insert_impl_trait_name(&mut self, id: NodeId, name: Symbol) {
                self.resolver.insert_impl_trait_name(id, name)
            }

            fn resolve_dollar_crates(&mut self) {
                self.resolver.resolve_dollar_crates()
            }

            fn register_glob_delegation(&mut self, invoc_id: LocalExpnId) {
                self.resolver.register_glob_delegation(invoc_id)
            }

            fn invocation_parent(&self, id: LocalExpnId) -> LocalDefId {
                self.resolver.invocation_parent(id)
            }

            fn expansion_cause(&self) -> Option<Span> {
                self.current_expansion.id.expansion_cause()
            }
        }
    };
}

use crate::ext_ctxt_def::{LintStoreExpand, LintStoreExpandDyn};

