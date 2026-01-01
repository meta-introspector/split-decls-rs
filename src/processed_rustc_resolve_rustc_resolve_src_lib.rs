/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_resolve_src_lib_UNPARSEABLE_0001
/* FP:lib.rs-0002 */ // This crate is responsible for the part of name resolution that doesn't require type checker.
/* FP:lib.rs-0003 */ //
/* FP:lib.rs-0004 */ // Module structure of the crate is built here.
/* FP:lib.rs-0005 */ // Paths in macros, imports, expressions, types, patterns are resolved here.
/* FP:lib.rs-0006 */ // Label and lifetime names are resolved here as well.
/* FP:lib.rs-0007 */ //
/* FP:lib.rs-0008 */ // Type-relative name resolution (methods, fields, associated items) happens in `rustc_hir_analysis`.
/* FP:lib.rs-0009 */ 
/* FP:lib.rs-0010 */ // tidy-alphabetical-start
/* FP:lib.rs-0011 */ #[allow(internal_features)]
/* FP:lib.rs-0012 */ #[allow(rustc::diagnostic_outside_of_impl)]
/* FP:lib.rs-0013 */ #[allow(rustc::untranslatable_diagnostic)]
/* FP:lib.rs-0014 */ #[doc(html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")]
/* FP:lib.rs-0015 */ #[doc(rust_logo)]
/* FP:lib.rs-0016 */ #[feature(arbitrary_self_types)]
/* FP:lib.rs-0017 */ #[feature(assert_matches)]
/* FP:lib.rs-0018 */ #[feature(box_patterns)]
/* FP:lib.rs-0019 */ #[feature(decl_macro)]
/* FP:lib.rs-0020 */ #[feature(default_field_values)]
/* FP:lib.rs-0021 */ #[feature(if_let_guard)]
/* FP:lib.rs-0022 */ #[feature(iter_intersperse)]
/* FP:lib.rs-0023 */ #[feature(rustc_attrs)]
/* FP:lib.rs-0024 */ #[feature(rustdoc_internals)]
/* FP:lib.rs-0025 */ #[recursion_limit = "256"]
/* FP:lib.rs-0026 */ // tidy-alphabetical-end
/* FP:lib.rs-0027 */ 
/* FP:lib.rs-0028 */ use std::cell::{Cell, Ref, RefCell};
/* FP:lib.rs-0029 */ use std::collections::BTreeSet;
/* FP:lib.rs-0030 */ use std::fmt;
/* FP:lib.rs-0031 */ use std::sync::Arc;
/* FP:lib.rs-0032 */ 
/* FP:lib.rs-0033 */ use diagnostics::{ImportSuggestion, LabelSuggestion, Suggestion};
/* FP:lib.rs-0034 */ use effective_visibilities::EffectiveVisibilitiesVisitor;
/* FP:lib.rs-0035 */ use errors::{ParamKindInEnumDiscriminant, ParamKindInNonTrivialAnonConst};
/* FP:lib.rs-0036 */ use imports::{Import, ImportData, ImportKind, NameResolution, PendingBinding};
/* FP:lib.rs-0037 */ use late::{
/* FP:lib.rs-0038 */     ForwardGenericParamBanReason, HasGenericParams, PathSource, PatternSource,
/* FP:lib.rs-0039 */     UnnecessaryQualification,
/* FP:lib.rs-0040 */ };
/* FP:lib.rs-0041 */ use macros::{MacroRulesBinding, MacroRulesScope, MacroRulesScopeRef};
/* FP:lib.rs-0042 */ use rustc_arena::{DroplessArena, TypedArena};
/* FP:lib.rs-0043 */ use crate::rustc_complete::node_id::NodeMap;
/* FP:lib.rs-0044 */ use crate::rustc_complete::{
/* FP:lib.rs-0045 */     self as ast, AngleBracketedArg, CRATE_NODE_ID, Crate, Expr, ExprKind, GenericArg, GenericArgs,
/* FP:lib.rs-0046 */     LitKind, NodeId, Path, attr,
/* FP:lib.rs-0047 */ };
/* FP:lib.rs-0048 */ use crate::rustc_data_structures::fx::{FxHashMap, FxHashSet, FxIndexMap, FxIndexSet};
/* FP:lib.rs-0049 */ use crate::rustc_data_structures::intern::Interned;
/* FP:lib.rs-0050 */ use crate::rustc_data_structures::steal::Steal;
/* FP:lib.rs-0051 */ use crate::rustc_data_structures::sync::{FreezeReadGuard, FreezeWriteGuard};
/* FP:lib.rs-0052 */ use crate::rustc_data_structures::unord::{UnordMap, UnordSet};
/* FP:lib.rs-0053 */ use crate::rustc_complete::{Applicability, Diag, ErrCode, ErrorGuaranteed, LintBuffer};
/* FP:lib.rs-0054 */ use crate::rustc_expand::base::{DeriveResolution, SyntaxExtension, SyntaxExtensionKind};
/* FP:lib.rs-0055 */ use crate::rustc_feature::BUILTIN_ATTRIBUTES;
/* FP:lib.rs-0056 */ use crate::rustc_complete::attrs::StrippedCfgItem;
/* FP:lib.rs-0057 */ use crate::rustc_complete::def::Namespace::{self, *};
/* FP:lib.rs-0058 */ use crate::rustc_complete::def::{
/* FP:lib.rs-0059 */     self, CtorOf, DefKind, DocLinkResMap, LifetimeRes, MacroKinds, NonMacroAttrKind, PartialRes,
/* FP:lib.rs-0060 */     PerNS,
/* FP:lib.rs-0061 */ };
/* FP:lib.rs-0062 */ use crate::rustc_complete::def_id::{CRATE_DEF_ID, CrateNum, DefId, LOCAL_CRATE, LocalDefId, LocalDefIdMap};
/* FP:lib.rs-0063 */ use crate::rustc_complete::definitions::DisambiguatorState;
/* FP:lib.rs-0064 */ use crate::rustc_complete::{PrimTy, TraitCandidate};
/* FP:lib.rs-0065 */ use crate::rustc_index::bit_set::DenseBitSet;
/* FP:lib.rs-0066 */ use crate::rustc_metadata::creader::CStore;
/* FP:lib.rs-0067 */ use crate::rustc_complete::metadata::ModChild;
/* FP:lib.rs-0068 */ use crate::rustc_complete::middle::privacy::EffectiveVisibilities;
/* FP:lib.rs-0069 */ use crate::rustc_complete::query::Providers;
/* FP:lib.rs-0070 */ use crate::rustc_complete::span_bug;
/* FP:lib.rs-0071 */ use crate::rustc_complete::ty::{
/* FP:lib.rs-0072 */     self, DelegationFnSig, Feed, MainDefinition, RegisteredTools, ResolverAstLowering,
/* FP:lib.rs-0073 */     ResolverGlobalCtxt, TyCtxt, TyCtxtFeed, Visibility,
/* FP:lib.rs-0074 */ };
/* FP:lib.rs-0075 */ use rustc_query_system::ich::StableHashingContext;
/* FP:lib.rs-0076 */ use crate::rustc_complete::lint::BuiltinLintDiag;
/* FP:lib.rs-0077 */ use crate::rustc_complete::lint::builtin::PRIVATE_MACRO_USE;
/* FP:lib.rs-0078 */ use crate::rustc_complete::hygiene::{ExpnId, LocalExpnId, MacroKind, SyntaxContext, Transparency};
/* FP:lib.rs-0079 */ use crate::rustc_complete::{DUMMY_SP, Ident, Macros20NormalizedIdent, Span, Symbol, kw, sym};
/* FP:lib.rs-0080 */ use smallvec::{SmallVec, smallvec};
/* FP:lib.rs-0081 */ use tracing::debug;
/* FP:lib.rs-0082 */ 
/* FP:lib.rs-0083 */ type Res = def::Res<NodeId>;
/* FP:lib.rs-0084 */ 
/* FP:lib.rs-0096 */ 
/* FP:lib.rs-0097 */ pub use macros::registered_tools_ast;
/* FP:lib.rs-0098 */ 
/* FP:lib.rs-0099 */ rustc_fluent_macro::fluent_messages! { "../messages.ftl" }
/* FP:lib.rs-0100 */ 
/* FP:lib.rs-0101 */ #[derive(Debug)]
/* FP:lib.rs-0102 */ enum Weak {
/* FP:lib.rs-0103 */     Yes,
/* FP:lib.rs-0104 */     No,
/* FP:lib.rs-0105 */ }
/* FP:lib.rs-0106 */ 
/* FP:lib.rs-0107 */ #[derive(Copy, Clone, PartialEq, Debug)]
/* FP:lib.rs-0108 */ enum Determinacy {
/* FP:lib.rs-0109 */     Determined,
/* FP:lib.rs-0110 */     Undetermined,
/* FP:lib.rs-0111 */ }
/* FP:lib.rs-0112 */ 
/* FP:lib.rs-0113 */ impl Determinacy {
/* FP:lib.rs-0114 */     fn determined(determined: bool) -> Determinacy {
/* FP:lib.rs-0115 */         if determined { Determinacy::Determined } else { Determinacy::Undetermined }
/* FP:lib.rs-0116 */     }
/* FP:lib.rs-0117 */ }
/* FP:lib.rs-0118 */ 
/* FP:lib.rs-0119 */ /// A specific scope in which a name can be looked up.
/* FP:lib.rs-0120 */ #[derive(Clone, Copy, Debug)]
/* FP:lib.rs-0121 */ enum Scope<'ra> {
/* FP:lib.rs-0122 */     /// Inert attributes registered by derive macros.
/* FP:lib.rs-0123 */     DeriveHelpers(LocalExpnId),
/* FP:lib.rs-0124 */     /// Inert attributes registered by derive macros, but used before they are actually declared.
/* FP:lib.rs-0125 */     /// This scope will exist until the compatibility lint `LEGACY_DERIVE_HELPERS`
/* FP:lib.rs-0126 */     /// is turned into a hard error.
/* FP:lib.rs-0127 */     DeriveHelpersCompat,
/* FP:lib.rs-0128 */     /// Textual `let`-like scopes introduced by `macro_rules!` items.
/* FP:lib.rs-0129 */     MacroRules(MacroRulesScopeRef<'ra>),
/* FP:lib.rs-0130 */     /// Names declared in the given module.
/* FP:lib.rs-0131 */     /// The node ID is for reporting the `PROC_MACRO_DERIVE_RESOLUTION_FALLBACK`
/* FP:lib.rs-0132 */     /// lint if it should be reported.
/* FP:lib.rs-0133 */     Module(Module<'ra>, Option<NodeId>),
/* FP:lib.rs-0134 */     /// Names introduced by `#[macro_use]` attributes on `extern crate` items.
/* FP:lib.rs-0135 */     MacroUsePrelude,
/* FP:lib.rs-0136 */     /// Built-in attributes.
/* FP:lib.rs-0137 */     BuiltinAttrs,
/* FP:lib.rs-0138 */     /// Extern prelude names introduced by `extern crate` items.
/* FP:lib.rs-0139 */     ExternPreludeItems,
/* FP:lib.rs-0140 */     /// Extern prelude names introduced by `--extern` flags.
/* FP:lib.rs-0141 */     ExternPreludeFlags,
/* FP:lib.rs-0142 */     /// Tool modules introduced with `#[register_tool]`.
/* FP:lib.rs-0143 */     ToolPrelude,
/* FP:lib.rs-0144 */     /// Standard library prelude introduced with an internal `#[prelude_import]` import.
/* FP:lib.rs-0145 */     StdLibPrelude,
/* FP:lib.rs-0146 */     /// Built-in types.
/* FP:lib.rs-0147 */     BuiltinTypes,
/* FP:lib.rs-0148 */ }
/* FP:lib.rs-0149 */ 
/* FP:lib.rs-0150 */ /// Names from different contexts may want to visit different subsets of all specific scopes
/* FP:lib.rs-0151 */ /// with different restrictions when looking up the resolution.
/* FP:lib.rs-0152 */ #[derive(Clone, Copy, Debug)]
/* FP:lib.rs-0153 */ enum ScopeSet<'ra> {
/* FP:lib.rs-0154 */     /// All scopes with the given namespace.
/* FP:lib.rs-0155 */     All(Namespace),
/* FP:lib.rs-0156 */     /// A module, then extern prelude (used for mixed 2015-2018 mode in macros).
/* FP:lib.rs-0157 */     ModuleAndExternPrelude(Namespace, Module<'ra>),
/* FP:lib.rs-0158 */     /// Just two extern prelude scopes.
/* FP:lib.rs-0159 */     ExternPrelude,
/* FP:lib.rs-0160 */     /// Same as `All(MacroNS)`, but with the given macro kind restriction.
/* FP:lib.rs-0161 */     Macro(MacroKind),
/* FP:lib.rs-0162 */ }
/* FP:lib.rs-0163 */ 
/* FP:lib.rs-0164 */ /// Everything you need to know about a name's location to resolve it.
/* FP:lib.rs-0165 */ /// Serves as a starting point for the scope visitor.
/* FP:lib.rs-0166 */ /// This struct is currently used only for early resolution (imports and macros),
/* FP:lib.rs-0167 */ /// but not for late resolution yet.
/* FP:lib.rs-0168 */ #[derive(Clone, Copy, Debug)]
/* FP:lib.rs-0169 */ struct ParentScope<'ra> {
/* FP:lib.rs-0170 */     module: Module<'ra>,
/* FP:lib.rs-0171 */     expansion: LocalExpnId,
/* FP:lib.rs-0172 */     macro_rules: MacroRulesScopeRef<'ra>,
/* FP:lib.rs-0173 */     derives: &'ra [ast::Path],
/* FP:lib.rs-0174 */ }
/* FP:lib.rs-0175 */ 
/* FP:lib.rs-0176 */ impl<'ra> ParentScope<'ra> {
/* FP:lib.rs-0177 */     /// Creates a parent scope with the passed argument used as the module scope component,
/* FP:lib.rs-0178 */     /// and other scope components set to default empty values.
/* FP:lib.rs-0179 */     fn module(module: Module<'ra>, arenas: &'ra ResolverArenas<'ra>) -> ParentScope<'ra> {
/* FP:lib.rs-0180 */         ParentScope {
/* FP:lib.rs-0181 */             module,
/* FP:lib.rs-0182 */             expansion: LocalExpnId::ROOT,
/* FP:lib.rs-0183 */             macro_rules: arenas.alloc_macro_rules_scope(MacroRulesScope::Empty),
/* FP:lib.rs-0184 */             derives: &[],
/* FP:lib.rs-0185 */         }
/* FP:lib.rs-0186 */     }
/* FP:lib.rs-0187 */ }
/* FP:lib.rs-0188 */ 
/* FP:lib.rs-0189 */ #[derive(Copy, Debug, Clone)]
/* FP:lib.rs-0190 */ struct InvocationParent {
/* FP:lib.rs-0191 */     parent_def: LocalDefId,
/* FP:lib.rs-0192 */     impl_trait_context: ImplTraitContext,
/* FP:lib.rs-0193 */     in_attr: bool,
/* FP:lib.rs-0194 */ }
/* FP:lib.rs-0195 */ 
/* FP:lib.rs-0196 */ impl InvocationParent {
/* FP:lib.rs-0197 */     const ROOT: Self = Self {
/* FP:lib.rs-0198 */         parent_def: CRATE_DEF_ID,
/* FP:lib.rs-0199 */         impl_trait_context: ImplTraitContext::Existential,
/* FP:lib.rs-0200 */         in_attr: false,
/* FP:lib.rs-0201 */     };
/* FP:lib.rs-0202 */ }
/* FP:lib.rs-0203 */ 
/* FP:lib.rs-0204 */ #[derive(Copy, Debug, Clone)]
/* FP:lib.rs-0205 */ enum ImplTraitContext {
/* FP:lib.rs-0206 */     Existential,
/* FP:lib.rs-0207 */     Universal,
/* FP:lib.rs-0208 */     InBinding,
/* FP:lib.rs-0209 */ }
/* FP:lib.rs-0210 */ 
/* FP:lib.rs-0211 */ /// Used for tracking import use types which will be used for redundant import checking.
/* FP:lib.rs-0212 */ ///
/* FP:lib.rs-0213 */ /// ### Used::Scope Example
/* FP:lib.rs-0214 */ ///
/* FP:lib.rs-0215 */ /// ```rust,compile_fail
/* FP:lib.rs-0216 */ /// #[deny(redundant_imports)]
/* FP:lib.rs-0217 */ /// use std::mem::drop;
/* FP:lib.rs-0218 */ /// fn main() {
/* FP:lib.rs-0219 */ ///     let s = Box::new(32);
/* FP:lib.rs-0220 */ ///     drop(s);
/* FP:lib.rs-0221 */ /// }
/* FP:lib.rs-0222 */ /// ```
/* FP:lib.rs-0223 */ ///
/* FP:lib.rs-0224 */ /// Used::Other is for other situations like module-relative uses.
/* FP:lib.rs-0225 */ #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
/* FP:lib.rs-0226 */ enum Used {
/* FP:lib.rs-0227 */     Scope,
/* FP:lib.rs-0228 */     Other,
/* FP:lib.rs-0229 */ }
/* FP:lib.rs-0230 */ 
/* FP:lib.rs-0231 */ #[derive(Debug)]
/* FP:lib.rs-0232 */ struct BindingError {
/* FP:lib.rs-0233 */     name: Ident,
/* FP:lib.rs-0234 */     origin: Vec<(Span, ast::Pat)>,
/* FP:lib.rs-0235 */     target: Vec<ast::Pat>,
/* FP:lib.rs-0236 */     could_be_path: bool,
/* FP:lib.rs-0237 */ }
/* FP:lib.rs-0238 */ 
/* FP:lib.rs-0239 */ #[derive(Debug)]
/* FP:lib.rs-0240 */ enum ResolutionError<'ra> {
/* FP:lib.rs-0241 */     /// Error E0401: can't use type or const parameters from outer item.
/* FP:lib.rs-0242 */     GenericParamsFromOuterItem(Res, HasGenericParams, DefKind),
/* FP:lib.rs-0243 */     /// Error E0403: the name is already used for a type or const parameter in this generic
/* FP:lib.rs-0244 */     /// parameter list.
/* FP:lib.rs-0245 */     NameAlreadyUsedInParameterList(Ident, Span),
/* FP:lib.rs-0246 */     /// Error E0407: method is not a member of trait.
/* FP:lib.rs-0247 */     MethodNotMemberOfTrait(Ident, String, Option<Symbol>),
/* FP:lib.rs-0248 */     /// Error E0437: type is not a member of trait.
/* FP:lib.rs-0249 */     TypeNotMemberOfTrait(Ident, String, Option<Symbol>),
/* FP:lib.rs-0250 */     /// Error E0438: const is not a member of trait.
/* FP:lib.rs-0251 */     ConstNotMemberOfTrait(Ident, String, Option<Symbol>),
/* FP:lib.rs-0252 */     /// Error E0408: variable `{}` is not bound in all patterns.
/* FP:lib.rs-0253 */     VariableNotBoundInPattern(BindingError, ParentScope<'ra>),
/* FP:lib.rs-0254 */     /// Error E0409: variable `{}` is bound in inconsistent ways within the same match arm.
/* FP:lib.rs-0255 */     VariableBoundWithDifferentMode(Ident, Span),
/* FP:lib.rs-0256 */     /// Error E0415: identifier is bound more than once in this parameter list.
/* FP:lib.rs-0257 */     IdentifierBoundMoreThanOnceInParameterList(Ident),
/* FP:lib.rs-0258 */     /// Error E0416: identifier is bound more than once in the same pattern.
/* FP:lib.rs-0259 */     IdentifierBoundMoreThanOnceInSamePattern(Ident),
/* FP:lib.rs-0260 */     /// Error E0426: use of undeclared label.
/* FP:lib.rs-0261 */     UndeclaredLabel { name: Symbol, suggestion: Option<LabelSuggestion> },
/* FP:lib.rs-0262 */     /// Error E0429: `self` imports are only allowed within a `{ }` list.
/* FP:lib.rs-0263 */     SelfImportsOnlyAllowedWithin { root: bool, span_with_rename: Span },
/* FP:lib.rs-0264 */     /// Error E0430: `self` import can only appear once in the list.
/* FP:lib.rs-0265 */     SelfImportCanOnlyAppearOnceInTheList,
/* FP:lib.rs-0266 */     /// Error E0431: `self` import can only appear in an import list with a non-empty prefix.
/* FP:lib.rs-0267 */     SelfImportOnlyInImportListWithNonEmptyPrefix,
/* FP:lib.rs-0268 */     /// Error E0433: failed to resolve.
/* FP:lib.rs-0269 */     FailedToResolve {
/* FP:lib.rs-0270 */         segment: Option<Symbol>,
/* FP:lib.rs-0271 */         label: String,
/* FP:lib.rs-0272 */         suggestion: Option<Suggestion>,
/* FP:lib.rs-0273 */         module: Option<ModuleOrUniformRoot<'ra>>,
/* FP:lib.rs-0274 */     },
/* FP:lib.rs-0275 */     /// Error E0434: can't capture dynamic environment in a fn item.
/* FP:lib.rs-0276 */     CannotCaptureDynamicEnvironmentInFnItem,
/* FP:lib.rs-0277 */     /// Error E0435: attempt to use a non-constant value in a constant.
/* FP:lib.rs-0278 */     AttemptToUseNonConstantValueInConstant {
/* FP:lib.rs-0279 */         ident: Ident,
/* FP:lib.rs-0280 */         suggestion: &'static str,
/* FP:lib.rs-0281 */         current: &'static str,
/* FP:lib.rs-0282 */         type_span: Option<Span>,
/* FP:lib.rs-0283 */     },
/* FP:lib.rs-0284 */     /// Error E0530: `X` bindings cannot shadow `Y`s.
/* FP:lib.rs-0285 */     BindingShadowsSomethingUnacceptable {
/* FP:lib.rs-0286 */         shadowing_binding: PatternSource,
/* FP:lib.rs-0287 */         name: Symbol,
/* FP:lib.rs-0288 */         participle: &'static str,
/* FP:lib.rs-0289 */         article: &'static str,
/* FP:lib.rs-0290 */         shadowed_binding: Res,
/* FP:lib.rs-0291 */         shadowed_binding_span: Span,
/* FP:lib.rs-0292 */     },
/* FP:lib.rs-0293 */     /// Error E0128: generic parameters with a default cannot use forward-declared identifiers.
/* FP:lib.rs-0294 */     ForwardDeclaredGenericParam(Symbol, ForwardGenericParamBanReason),
/* FP:lib.rs-0295 */     // FIXME(generic_const_parameter_types): This should give custom output specifying it's only
/* FP:lib.rs-0296 */     // problematic to use *forward declared* parameters when the feature is enabled.
/* FP:lib.rs-0297 */     /// ERROR E0770: the type of const parameters must not depend on other generic parameters.
/* FP:lib.rs-0298 */     ParamInTyOfConstParam { name: Symbol },
/* FP:lib.rs-0299 */     /// generic parameters must not be used inside const evaluations.
/* FP:lib.rs-0300 */     ///
/* FP:lib.rs-0301 */     /// This error is only emitted when using `min_const_generics`.
/* FP:lib.rs-0302 */     ParamInNonTrivialAnonConst { name: Symbol, param_kind: ParamKindInNonTrivialAnonConst },
/* FP:lib.rs-0303 */     /// generic parameters must not be used inside enum discriminants.
/* FP:lib.rs-0304 */     ///
/* FP:lib.rs-0305 */     /// This error is emitted even with `generic_const_exprs`.
/* FP:lib.rs-0306 */     ParamInEnumDiscriminant { name: Symbol, param_kind: ParamKindInEnumDiscriminant },
/* FP:lib.rs-0307 */     /// Error E0735: generic parameters with a default cannot use `Self`
/* FP:lib.rs-0308 */     ForwardDeclaredSelf(ForwardGenericParamBanReason),
/* FP:lib.rs-0309 */     /// Error E0767: use of unreachable label
/* FP:lib.rs-0310 */     UnreachableLabel { name: Symbol, definition_span: Span, suggestion: Option<LabelSuggestion> },
/* FP:lib.rs-0311 */     /// Error E0323, E0324, E0325: mismatch between trait item and impl item.
/* FP:lib.rs-0312 */     TraitImplMismatch {
/* FP:lib.rs-0313 */         name: Ident,
/* FP:lib.rs-0314 */         kind: &'static str,
/* FP:lib.rs-0315 */         trait_path: String,
/* FP:lib.rs-0316 */         trait_item_span: Span,
/* FP:lib.rs-0317 */         code: ErrCode,
/* FP:lib.rs-0318 */     },
/* FP:lib.rs-0319 */     /// Error E0201: multiple impl items for the same trait item.
/* FP:lib.rs-0320 */     TraitImplDuplicate { name: Ident, trait_item_span: Span, old_span: Span },
/* FP:lib.rs-0321 */     /// Inline asm `sym` operand must refer to a `fn` or `static`.
/* FP:lib.rs-0322 */     InvalidAsmSym,
/* FP:lib.rs-0323 */     /// `self` used instead of `Self` in a generic parameter
/* FP:lib.rs-0324 */     LowercaseSelf,
/* FP:lib.rs-0325 */     /// A never pattern has a binding.
/* FP:lib.rs-0326 */     BindingInNeverPattern,
/* FP:lib.rs-0327 */ }
/* FP:lib.rs-0328 */ 
/* FP:lib.rs-0329 */ enum VisResolutionError<'a> {
/* FP:lib.rs-0330 */     Relative2018(Span, &'a ast::Path),
/* FP:lib.rs-0331 */     AncestorOnly(Span),
/* FP:lib.rs-0332 */     FailedToResolve(Span, String, Option<Suggestion>),
/* FP:lib.rs-0333 */     ExpectedFound(Span, String, Res),
/* FP:lib.rs-0334 */     Indeterminate(Span),
/* FP:lib.rs-0335 */     ModuleOnly(Span),
/* FP:lib.rs-0336 */ }
/* FP:lib.rs-0337 */ 
/* FP:lib.rs-0338 */ /// A minimal representation of a path segment. We use this in resolve because we synthesize 'path
/* FP:lib.rs-0339 */ /// segments' which don't have the rest of an AST or HIR `PathSegment`.
/* FP:lib.rs-0340 */ #[derive(Clone, Copy, Debug)]
/* FP:lib.rs-0341 */ struct Segment {
/* FP:lib.rs-0342 */     ident: Ident,
/* FP:lib.rs-0343 */     id: Option<NodeId>,
/* FP:lib.rs-0344 */     /// Signals whether this `PathSegment` has generic arguments. Used to avoid providing
/* FP:lib.rs-0345 */     /// nonsensical suggestions.
/* FP:lib.rs-0346 */     has_generic_args: bool,
/* FP:lib.rs-0347 */     /// Signals whether this `PathSegment` has lifetime arguments.
/* FP:lib.rs-0348 */     has_lifetime_args: bool,
/* FP:lib.rs-0349 */     args_span: Span,
/* FP:lib.rs-0350 */ }
/* FP:lib.rs-0351 */ 
/* FP:lib.rs-0352 */ impl Segment {
/* FP:lib.rs-0353 */     fn from_path(path: &Path) -> Vec<Segment> {
/* FP:lib.rs-0354 */         path.segments.iter().map(|s| s.into()).collect()
/* FP:lib.rs-0355 */     }
/* FP:lib.rs-0356 */ 
/* FP:lib.rs-0357 */     fn from_ident(ident: Ident) -> Segment {
/* FP:lib.rs-0358 */         Segment {
/* FP:lib.rs-0359 */             ident,
/* FP:lib.rs-0360 */             id: None,
/* FP:lib.rs-0361 */             has_generic_args: false,
/* FP:lib.rs-0362 */             has_lifetime_args: false,
/* FP:lib.rs-0363 */             args_span: DUMMY_SP,
/* FP:lib.rs-0364 */         }
/* FP:lib.rs-0365 */     }
/* FP:lib.rs-0366 */ 
/* FP:lib.rs-0367 */     fn from_ident_and_id(ident: Ident, id: NodeId) -> Segment {
/* FP:lib.rs-0368 */         Segment {
/* FP:lib.rs-0369 */             ident,
/* FP:lib.rs-0370 */             id: Some(id),
/* FP:lib.rs-0371 */             has_generic_args: false,
/* FP:lib.rs-0372 */             has_lifetime_args: false,
/* FP:lib.rs-0373 */             args_span: DUMMY_SP,
/* FP:lib.rs-0374 */         }
/* FP:lib.rs-0375 */     }
/* FP:lib.rs-0376 */ 
/* FP:lib.rs-0377 */     fn names_to_string(segments: &[Segment]) -> String {
/* FP:lib.rs-0378 */         names_to_string(segments.iter().map(|seg| seg.ident.name))
/* FP:lib.rs-0379 */     }
/* FP:lib.rs-0380 */ }
/* FP:lib.rs-0381 */ 
/* FP:lib.rs-0382 */ impl<'a> From<&'a ast::PathSegment> for Segment {
/* FP:lib.rs-0383 */     fn from(seg: &'a ast::PathSegment) -> Segment {
/* FP:lib.rs-0384 */         let has_generic_args = seg.args.is_some();
/* FP:lib.rs-0385 */         let (args_span, has_lifetime_args) = if let Some(args) = seg.args.as_deref() {
/* FP:lib.rs-0386 */             match args {
/* FP:lib.rs-0387 */                 GenericArgs::AngleBracketed(args) => {
/* FP:lib.rs-0388 */                     let found_lifetimes = args
/* FP:lib.rs-0389 */                         .args
/* FP:lib.rs-0390 */                         .iter()
/* FP:lib.rs-0391 */                         .any(|arg| matches!(arg, AngleBracketedArg::Arg(GenericArg::Lifetime(_))));
/* FP:lib.rs-0392 */                     (args.span, found_lifetimes)
/* FP:lib.rs-0393 */                 }
/* FP:lib.rs-0394 */                 GenericArgs::Parenthesized(args) => (args.span, true),
/* FP:lib.rs-0395 */                 GenericArgs::ParenthesizedElided(span) => (*span, true),
/* FP:lib.rs-0396 */             }
/* FP:lib.rs-0397 */         } else {
/* FP:lib.rs-0398 */             (DUMMY_SP, false)
/* FP:lib.rs-0399 */         };
/* FP:lib.rs-0400 */         Segment {
/* FP:lib.rs-0401 */             ident: seg.ident,
/* FP:lib.rs-0402 */             id: Some(seg.id),
/* FP:lib.rs-0403 */             has_generic_args,
/* FP:lib.rs-0404 */             has_lifetime_args,
/* FP:lib.rs-0405 */             args_span,
/* FP:lib.rs-0406 */         }
/* FP:lib.rs-0407 */     }
/* FP:lib.rs-0408 */ }
/* FP:lib.rs-0409 */ 
/* FP:lib.rs-0410 */ /// An intermediate resolution result.
/* FP:lib.rs-0411 */ ///
/* FP:lib.rs-0412 */ /// This refers to the thing referred by a name. The difference between `Res` and `Item` is that
/* FP:lib.rs-0413 */ /// items are visible in their whole block, while `Res`es only from the place they are defined
/* FP:lib.rs-0414 */ /// forward.
/* FP:lib.rs-0415 */ #[derive(Debug, Copy, Clone)]
/* FP:lib.rs-0416 */ enum LexicalScopeBinding<'ra> {
/* FP:lib.rs-0417 */     Item(NameBinding<'ra>),
/* FP:lib.rs-0418 */     Res(Res),
/* FP:lib.rs-0419 */ }
/* FP:lib.rs-0420 */ 
/* FP:lib.rs-0421 */ impl<'ra> LexicalScopeBinding<'ra> {
/* FP:lib.rs-0422 */     fn res(self) -> Res {
/* FP:lib.rs-0423 */         match self {
/* FP:lib.rs-0424 */             LexicalScopeBinding::Item(binding) => binding.res(),
/* FP:lib.rs-0425 */             LexicalScopeBinding::Res(res) => res,
/* FP:lib.rs-0426 */         }
/* FP:lib.rs-0427 */     }
/* FP:lib.rs-0428 */ }
/* FP:lib.rs-0429 */ 
/* FP:lib.rs-0430 */ #[derive(Copy, Clone, PartialEq, Debug)]
/* FP:lib.rs-0431 */ enum ModuleOrUniformRoot<'ra> {
/* FP:lib.rs-0432 */     /// Regular module.
/* FP:lib.rs-0433 */     Module(Module<'ra>),
/* FP:lib.rs-0434 */ 
/* FP:lib.rs-0435 */     /// Virtual module that denotes resolution in a module with fallback to extern prelude.
/* FP:lib.rs-0436 */     /// Used for paths starting with `::` coming from 2015 edition macros
/* FP:lib.rs-0437 */     /// used in 2018+ edition crates.
/* FP:lib.rs-0438 */     ModuleAndExternPrelude(Module<'ra>),
/* FP:lib.rs-0439 */ 
/* FP:lib.rs-0440 */     /// Virtual module that denotes resolution in extern prelude.
/* FP:lib.rs-0441 */     /// Used for paths starting with `::` on 2018 edition.
/* FP:lib.rs-0442 */     ExternPrelude,
/* FP:lib.rs-0443 */ 
/* FP:lib.rs-0444 */     /// Virtual module that denotes resolution in current scope.
/* FP:lib.rs-0445 */     /// Used only for resolving single-segment imports. The reason it exists is that import paths
/* FP:lib.rs-0446 */     /// are always split into two parts, the first of which should be some kind of module.
/* FP:lib.rs-0447 */     CurrentScope,
/* FP:lib.rs-0448 */ }
/* FP:lib.rs-0449 */ 
/* FP:lib.rs-0450 */ #[derive(Debug)]
/* FP:lib.rs-0451 */ enum PathResult<'ra> {
/* FP:lib.rs-0452 */     Module(ModuleOrUniformRoot<'ra>),
/* FP:lib.rs-0453 */     NonModule(PartialRes),
/* FP:lib.rs-0454 */     Indeterminate,
/* FP:lib.rs-0455 */     Failed {
/* FP:lib.rs-0456 */         span: Span,
/* FP:lib.rs-0457 */         label: String,
/* FP:lib.rs-0458 */         suggestion: Option<Suggestion>,
/* FP:lib.rs-0459 */         is_error_from_last_segment: bool,
/* FP:lib.rs-0460 */         /// The final module being resolved, for instance:
/* FP:lib.rs-0461 */         ///
/* FP:lib.rs-0462 */         /// ```compile_fail
/* FP:lib.rs-0463 */         /// mod a {
/* FP:lib.rs-0464 */         ///     mod b {
/* FP:lib.rs-0465 */         ///         mod c {}
/* FP:lib.rs-0466 */         ///     }
/* FP:lib.rs-0467 */         /// }
/* FP:lib.rs-0468 */         ///
/* FP:lib.rs-0469 */         /// use a::not_exist::c;
/* FP:lib.rs-0470 */         /// ```
/* FP:lib.rs-0471 */         ///
/* FP:lib.rs-0472 */         /// In this case, `module` will point to `a`.
/* FP:lib.rs-0473 */         module: Option<ModuleOrUniformRoot<'ra>>,
/* FP:lib.rs-0474 */         /// The segment name of target
/* FP:lib.rs-0475 */         segment_name: Symbol,
/* FP:lib.rs-0476 */         error_implied_by_parse_error: bool,
/* FP:lib.rs-0477 */     },
/* FP:lib.rs-0478 */ }
/* FP:lib.rs-0479 */ 
/* FP:lib.rs-0480 */ impl<'ra> PathResult<'ra> {
/* FP:lib.rs-0481 */     fn failed(
/* FP:lib.rs-0482 */         ident: Ident,
/* FP:lib.rs-0483 */         is_error_from_last_segment: bool,
/* FP:lib.rs-0484 */         finalize: bool,
/* FP:lib.rs-0485 */         error_implied_by_parse_error: bool,
/* FP:lib.rs-0486 */         module: Option<ModuleOrUniformRoot<'ra>>,
/* FP:lib.rs-0487 */         label_and_suggestion: impl FnOnce() -> (String, Option<Suggestion>),
/* FP:lib.rs-0488 */     ) -> PathResult<'ra> {
/* FP:lib.rs-0489 */         let (label, suggestion) =
/* FP:lib.rs-0490 */             if finalize { label_and_suggestion() } else { (String::new(), None) };
/* FP:lib.rs-0491 */         PathResult::Failed {
/* FP:lib.rs-0492 */             span: ident.span,
/* FP:lib.rs-0493 */             segment_name: ident.name,
/* FP:lib.rs-0494 */             label,
/* FP:lib.rs-0495 */             suggestion,
/* FP:lib.rs-0496 */             is_error_from_last_segment,
/* FP:lib.rs-0497 */             module,
/* FP:lib.rs-0498 */             error_implied_by_parse_error,
/* FP:lib.rs-0499 */         }
/* FP:lib.rs-0500 */     }
/* FP:lib.rs-0501 */ }
/* FP:lib.rs-0502 */ 
/* FP:lib.rs-0503 */ #[derive(Debug)]
/* FP:lib.rs-0504 */ enum ModuleKind {
/* FP:lib.rs-0505 */     /// An anonymous module; e.g., just a block.
/* FP:lib.rs-0506 */     ///
/* FP:lib.rs-0507 */     /// ```
/* FP:lib.rs-0508 */     /// fn main() {
/* FP:lib.rs-0509 */     ///     fn f() {} // (1)
/* FP:lib.rs-0510 */     ///     { // This is an anonymous module
/* FP:lib.rs-0511 */     ///         f(); // This resolves to (2) as we are inside the block.
/* FP:lib.rs-0512 */     ///         fn f() {} // (2)
/* FP:lib.rs-0513 */     ///     }
/* FP:lib.rs-0514 */     ///     f(); // Resolves to (1)
/* FP:lib.rs-0515 */     /// }
/* FP:lib.rs-0516 */     /// ```
/* FP:lib.rs-0517 */     Block,
/* FP:lib.rs-0518 */     /// Any module with a name.
/* FP:lib.rs-0519 */     ///
/* FP:lib.rs-0520 */     /// This could be:
/* FP:lib.rs-0521 */     ///
/* FP:lib.rs-0523 */     ///   or the crate root (which is conceptually a top-level module).
/* FP:lib.rs-0524 */     ///   The crate root will have `None` for the symbol.
/* FP:lib.rs-0525 */     /// * A trait or an enum (it implicitly contains associated types, methods and variant
/* FP:lib.rs-0526 */     ///   constructors).
/* FP:lib.rs-0527 */     Def(DefKind, DefId, Option<Symbol>),
/* FP:lib.rs-0528 */ }
/* FP:lib.rs-0529 */ 
/* FP:lib.rs-0530 */ impl ModuleKind {
/* FP:lib.rs-0531 */     /// Get name of the module.
/* FP:lib.rs-0532 */     fn name(&self) -> Option<Symbol> {
/* FP:lib.rs-0533 */         match *self {
/* FP:lib.rs-0534 */             ModuleKind::Block => None,
/* FP:lib.rs-0535 */             ModuleKind::Def(.., name) => name,
/* FP:lib.rs-0536 */         }
/* FP:lib.rs-0537 */     }
/* FP:lib.rs-0538 */ }
/* FP:lib.rs-0539 */ 
/* FP:lib.rs-0540 */ /// A key that identifies a binding in a given `Module`.
/* FP:lib.rs-0541 */ ///
/* FP:lib.rs-0542 */ /// Multiple bindings in the same module can have the same key (in a valid
/* FP:lib.rs-0543 */ /// program) if all but one of them come from glob imports.
/* FP:lib.rs-0544 */ #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
/* FP:lib.rs-0545 */ struct BindingKey {
/* FP:lib.rs-0546 */     /// The identifier for the binding, always the `normalize_to_macros_2_0` version of the
/* FP:lib.rs-0547 */     /// identifier.
/* FP:lib.rs-0548 */     ident: Macros20NormalizedIdent,
/* FP:lib.rs-0549 */     ns: Namespace,
/* FP:lib.rs-0550 */     /// When we add an underscore binding (with ident `_`) to some module, this field has
/* FP:lib.rs-0551 */     /// a non-zero value that uniquely identifies this binding in that module.
/* FP:lib.rs-0552 */     /// For non-underscore bindings this field is zero.
/* FP:lib.rs-0553 */     /// When a key is constructed for name lookup (as opposed to name definition), this field is
/* FP:lib.rs-0554 */     /// also zero, even for underscore names, so for underscores the lookup will never succeed.
/* FP:lib.rs-0555 */     disambiguator: u32,
/* FP:lib.rs-0556 */ }
/* FP:lib.rs-0557 */ 
/* FP:lib.rs-0558 */ impl BindingKey {
/* FP:lib.rs-0559 */     fn new(ident: Ident, ns: Namespace) -> Self {
/* FP:lib.rs-0560 */         BindingKey { ident: Macros20NormalizedIdent::new(ident), ns, disambiguator: 0 }
/* FP:lib.rs-0561 */     }
/* FP:lib.rs-0562 */ 
/* FP:lib.rs-0563 */     fn new_disambiguated(
/* FP:lib.rs-0564 */         ident: Ident,
/* FP:lib.rs-0565 */         ns: Namespace,
/* FP:lib.rs-0566 */         disambiguator: impl FnOnce() -> u32,
/* FP:lib.rs-0567 */     ) -> BindingKey {
/* FP:lib.rs-0568 */         let disambiguator = if ident.name == kw::Underscore { disambiguator() } else { 0 };
/* FP:lib.rs-0569 */         BindingKey { ident: Macros20NormalizedIdent::new(ident), ns, disambiguator }
/* FP:lib.rs-0570 */     }
/* FP:lib.rs-0571 */ }
/* FP:lib.rs-0572 */ 
/* FP:lib.rs-0573 */ type Resolutions<'ra> = RefCell<FxIndexMap<BindingKey, &'ra RefCell<NameResolution<'ra>>>>;
/* FP:lib.rs-0574 */ 
/* FP:lib.rs-0575 */ /// One node in the tree of modules.
/* FP:lib.rs-0576 */ ///
/* FP:lib.rs-0577 */ /// Note that a "module" in resolve is broader than a `mod` that you declare in Rust code. It may be one of these:
/* FP:lib.rs-0578 */ ///
/* FP:lib.rs-0579 */ /// * `mod`
/* FP:lib.rs-0580 */ /// * crate root (aka, top-level anonymous module)
/* FP:lib.rs-0581 */ /// * `enum`
/* FP:lib.rs-0582 */ /// * `trait`
/* FP:lib.rs-0583 */ /// * curly-braced block with statements
/* FP:lib.rs-0584 */ ///
/* FP:lib.rs-0585 */ /// You can use [`ModuleData::kind`] to determine the kind of module this is.
/* FP:lib.rs-0586 */ struct ModuleData<'ra> {
/* FP:lib.rs-0587 */     /// The direct parent module (it may not be a `mod`, however).
/* FP:lib.rs-0588 */     parent: Option<Module<'ra>>,
/* FP:lib.rs-0589 */     /// What kind of module this is, because this may not be a `mod`.
/* FP:lib.rs-0590 */     kind: ModuleKind,
/* FP:lib.rs-0591 */ 
/* FP:lib.rs-0592 */     /// Mapping between names and their (possibly in-progress) resolutions in this module.
/* FP:lib.rs-0593 */     /// Resolutions in modules from other crates are not populated until accessed.
/* FP:lib.rs-0594 */     lazy_resolutions: Resolutions<'ra>,
/* FP:lib.rs-0595 */     /// True if this is a module from other crate that needs to be populated on access.
/* FP:lib.rs-0596 */     populate_on_access: Cell<bool>,
/* FP:lib.rs-0597 */     /// Used to disambiguate underscore items (`const _: T = ...`) in the module.
/* FP:lib.rs-0598 */     underscore_disambiguator: Cell<u32>,
/* FP:lib.rs-0599 */ 
/* FP:lib.rs-0600 */     /// Macro invocations that can expand into items in this module.
/* FP:lib.rs-0601 */     unexpanded_invocations: RefCell<FxHashSet<LocalExpnId>>,
/* FP:lib.rs-0602 */ 
/* FP:lib.rs-0603 */     /// Whether `#[no_implicit_prelude]` is active.
/* FP:lib.rs-0604 */     no_implicit_prelude: bool,
/* FP:lib.rs-0605 */ 
/* FP:lib.rs-0606 */     glob_importers: RefCell<Vec<Import<'ra>>>,
/* FP:lib.rs-0607 */     globs: RefCell<Vec<Import<'ra>>>,
/* FP:lib.rs-0608 */ 
/* FP:lib.rs-0609 */     /// Used to memoize the traits in this module for faster searches through all traits in scope.
/* FP:lib.rs-0610 */     traits:
/* FP:lib.rs-0611 */         RefCell<Option<Box<[(Macros20NormalizedIdent, NameBinding<'ra>, Option<Module<'ra>>)]>>>,
/* FP:lib.rs-0612 */ 
/* FP:lib.rs-0613 */     /// Span of the module itself. Used for error reporting.
/* FP:lib.rs-0614 */     span: Span,
/* FP:lib.rs-0615 */ 
/* FP:lib.rs-0616 */     expansion: ExpnId,
/* FP:lib.rs-0617 */ 
/* FP:lib.rs-0618 */     /// Binding for implicitly declared names that come with a module,
/* FP:lib.rs-0619 */     /// like `self` (not yet used), or `crate`/`$crate` (for root modules).
/* FP:lib.rs-0620 */     self_binding: Option<NameBinding<'ra>>,
/* FP:lib.rs-0621 */ }
/* FP:lib.rs-0622 */ 
/* FP:lib.rs-0623 */ /// All modules are unique and allocated on a same arena,
/* FP:lib.rs-0624 */ /// so we can use referential equality to compare them.
/* FP:lib.rs-0625 */ #[derive(Clone, Copy, PartialEq, Eq, Hash)]
/* FP:lib.rs-0626 */ #[rustc_pass_by_value]
/* FP:lib.rs-0627 */ struct Module<'ra>(Interned<'ra, ModuleData<'ra>>);
/* FP:lib.rs-0628 */ 
/* FP:lib.rs-0629 */ // Allows us to use Interned without actually enforcing (via Hash/PartialEq/...) uniqueness of the
/* FP:lib.rs-0630 */ // contained data.
/* FP:lib.rs-0631 */ // FIXME: We may wish to actually have at least debug-level assertions that Interned's guarantees
/* FP:lib.rs-0632 */ // are upheld.
/* FP:lib.rs-0633 */ impl std::hash::Hash for ModuleData<'_> {
/* FP:lib.rs-0634 */     fn hash<H>(&self, _: &mut H)
/* FP:lib.rs-0635 */     where
/* FP:lib.rs-0636 */         H: std::hash::Hasher,
/* FP:lib.rs-0637 */     {
/* FP:lib.rs-0638 */         unreachable!()
/* FP:lib.rs-0639 */     }
/* FP:lib.rs-0640 */ }
/* FP:lib.rs-0641 */ 
/* FP:lib.rs-0642 */ impl<'ra> ModuleData<'ra> {
/* FP:lib.rs-0643 */     fn new(
/* FP:lib.rs-0644 */         parent: Option<Module<'ra>>,
/* FP:lib.rs-0645 */         kind: ModuleKind,
/* FP:lib.rs-0646 */         expansion: ExpnId,
/* FP:lib.rs-0647 */         span: Span,
/* FP:lib.rs-0648 */         no_implicit_prelude: bool,
/* FP:lib.rs-0649 */         self_binding: Option<NameBinding<'ra>>,
/* FP:lib.rs-0650 */     ) -> Self {
/* FP:lib.rs-0651 */         let is_foreign = match kind {
/* FP:lib.rs-0652 */             ModuleKind::Def(_, def_id, _) => !def_id.is_local(),
/* FP:lib.rs-0653 */             ModuleKind::Block => false,
/* FP:lib.rs-0654 */         };
/* FP:lib.rs-0655 */         ModuleData {
/* FP:lib.rs-0656 */             parent,
/* FP:lib.rs-0657 */             kind,
/* FP:lib.rs-0658 */             lazy_resolutions: Default::default(),
/* FP:lib.rs-0659 */             populate_on_access: Cell::new(is_foreign),
/* FP:lib.rs-0660 */             underscore_disambiguator: Cell::new(0),
/* FP:lib.rs-0661 */             unexpanded_invocations: Default::default(),
/* FP:lib.rs-0662 */             no_implicit_prelude,
/* FP:lib.rs-0663 */             glob_importers: RefCell::new(Vec::new()),
/* FP:lib.rs-0664 */             globs: RefCell::new(Vec::new()),
/* FP:lib.rs-0665 */             traits: RefCell::new(None),
/* FP:lib.rs-0666 */             span,
/* FP:lib.rs-0667 */             expansion,
/* FP:lib.rs-0668 */             self_binding,
/* FP:lib.rs-0669 */         }
/* FP:lib.rs-0670 */     }
/* FP:lib.rs-0671 */ }
/* FP:lib.rs-0672 */ 
/* FP:lib.rs-0673 */ impl<'ra> Module<'ra> {
/* FP:lib.rs-0674 */     fn for_each_child<'tcx, R: AsRef<Resolver<'ra, 'tcx>>>(
/* FP:lib.rs-0675 */         self,
/* FP:lib.rs-0676 */         resolver: &R,
/* FP:lib.rs-0677 */         mut f: impl FnMut(&R, Macros20NormalizedIdent, Namespace, NameBinding<'ra>),
/* FP:lib.rs-0678 */     ) {
/* FP:lib.rs-0679 */         for (key, name_resolution) in resolver.as_ref().resolutions(self).borrow().iter() {
/* FP:lib.rs-0680 */             if let Some(binding) = name_resolution.borrow().best_binding() {
/* FP:lib.rs-0681 */                 f(resolver, key.ident, key.ns, binding);
/* FP:lib.rs-0682 */             }
/* FP:lib.rs-0683 */         }
/* FP:lib.rs-0684 */     }
/* FP:lib.rs-0685 */ 
/* FP:lib.rs-0686 */     fn for_each_child_mut<'tcx, R: AsMut<Resolver<'ra, 'tcx>>>(
/* FP:lib.rs-0687 */         self,
/* FP:lib.rs-0688 */         resolver: &mut R,
/* FP:lib.rs-0689 */         mut f: impl FnMut(&mut R, Macros20NormalizedIdent, Namespace, NameBinding<'ra>),
/* FP:lib.rs-0690 */     ) {
/* FP:lib.rs-0691 */         for (key, name_resolution) in resolver.as_mut().resolutions(self).borrow().iter() {
/* FP:lib.rs-0692 */             if let Some(binding) = name_resolution.borrow().best_binding() {
/* FP:lib.rs-0693 */                 f(resolver, key.ident, key.ns, binding);
/* FP:lib.rs-0694 */             }
/* FP:lib.rs-0695 */         }
/* FP:lib.rs-0696 */     }
/* FP:lib.rs-0697 */ 
/* FP:lib.rs-0698 */     /// This modifies `self` in place. The traits will be stored in `self.traits`.
/* FP:lib.rs-0699 */     fn ensure_traits<'tcx>(self, resolver: &impl AsRef<Resolver<'ra, 'tcx>>) {
/* FP:lib.rs-0700 */         let mut traits = self.traits.borrow_mut();
/* FP:lib.rs-0701 */         if traits.is_none() {
/* FP:lib.rs-0702 */             let mut collected_traits = Vec::new();
/* FP:lib.rs-0703 */             self.for_each_child(resolver, |r, name, ns, binding| {
/* FP:lib.rs-0704 */                 if ns != TypeNS {
/* FP:lib.rs-0705 */                     return;
/* FP:lib.rs-0706 */                 }
/* FP:lib.rs-0707 */                 if let Res::Def(DefKind::Trait | DefKind::TraitAlias, def_id) = binding.res() {
/* FP:lib.rs-0708 */                     collected_traits.push((name, binding, r.as_ref().get_module(def_id)))
/* FP:lib.rs-0709 */                 }
/* FP:lib.rs-0710 */             });
/* FP:lib.rs-0711 */             *traits = Some(collected_traits.into_boxed_slice());
/* FP:lib.rs-0712 */         }
/* FP:lib.rs-0713 */     }
/* FP:lib.rs-0714 */ 
/* FP:lib.rs-0715 */     fn res(self) -> Option<Res> {
/* FP:lib.rs-0716 */         match self.kind {
/* FP:lib.rs-0717 */             ModuleKind::Def(kind, def_id, _) => Some(Res::Def(kind, def_id)),
/* FP:lib.rs-0718 */             _ => None,
/* FP:lib.rs-0719 */         }
/* FP:lib.rs-0720 */     }
/* FP:lib.rs-0721 */ 
/* FP:lib.rs-0722 */     fn def_id(self) -> DefId {
/* FP:lib.rs-0723 */         self.opt_def_id().expect("`ModuleData::def_id` is called on a block module")
/* FP:lib.rs-0724 */     }
/* FP:lib.rs-0725 */ 
/* FP:lib.rs-0726 */     fn opt_def_id(self) -> Option<DefId> {
/* FP:lib.rs-0727 */         match self.kind {
/* FP:lib.rs-0728 */             ModuleKind::Def(_, def_id, _) => Some(def_id),
/* FP:lib.rs-0729 */             _ => None,
/* FP:lib.rs-0730 */         }
/* FP:lib.rs-0731 */     }
/* FP:lib.rs-0732 */ 
/* FP:lib.rs-0733 */     // `self` resolves to the first module ancestor that `is_normal`.
/* FP:lib.rs-0734 */     fn is_normal(self) -> bool {
/* FP:lib.rs-0735 */         matches!(self.kind, ModuleKind::Def(DefKind::Mod, _, _))
/* FP:lib.rs-0736 */     }
/* FP:lib.rs-0737 */ 
/* FP:lib.rs-0738 */     fn is_trait(self) -> bool {
/* FP:lib.rs-0739 */         matches!(self.kind, ModuleKind::Def(DefKind::Trait, _, _))
/* FP:lib.rs-0740 */     }
/* FP:lib.rs-0741 */ 
/* FP:lib.rs-0742 */     fn nearest_item_scope(self) -> Module<'ra> {
/* FP:lib.rs-0743 */         match self.kind {
/* FP:lib.rs-0744 */             ModuleKind::Def(DefKind::Enum | DefKind::Trait, ..) => {
/* FP:lib.rs-0745 */                 self.parent.expect("enum or trait module without a parent")
/* FP:lib.rs-0746 */             }
/* FP:lib.rs-0747 */             _ => self,
/* FP:lib.rs-0748 */         }
/* FP:lib.rs-0749 */     }
/* FP:lib.rs-0750 */ 
/* FP:lib.rs-0751 */     /// The [`DefId`] of the nearest `mod` item ancestor (which may be this module).
/* FP:lib.rs-0752 */     /// This may be the crate root.
/* FP:lib.rs-0753 */     fn nearest_parent_mod(self) -> DefId {
/* FP:lib.rs-0754 */         match self.kind {
/* FP:lib.rs-0755 */             ModuleKind::Def(DefKind::Mod, def_id, _) => def_id,
/* FP:lib.rs-0756 */             _ => self.parent.expect("non-root module without parent").nearest_parent_mod(),
/* FP:lib.rs-0757 */         }
/* FP:lib.rs-0758 */     }
/* FP:lib.rs-0759 */ 
/* FP:lib.rs-0760 */     fn is_ancestor_of(self, mut other: Self) -> bool {
/* FP:lib.rs-0761 */         while self != other {
/* FP:lib.rs-0762 */             if let Some(parent) = other.parent {
/* FP:lib.rs-0763 */                 other = parent;
/* FP:lib.rs-0764 */             } else {
/* FP:lib.rs-0765 */                 return false;
/* FP:lib.rs-0766 */             }
/* FP:lib.rs-0767 */         }
/* FP:lib.rs-0768 */         true
/* FP:lib.rs-0769 */     }
/* FP:lib.rs-0770 */ }
/* FP:lib.rs-0771 */ 
/* FP:lib.rs-0772 */ impl<'ra> std::ops::Deref for Module<'ra> {
/* FP:lib.rs-0773 */     type Target = ModuleData<'ra>;
/* FP:lib.rs-0774 */ 
/* FP:lib.rs-0775 */     fn deref(&self) -> &Self::Target {
/* FP:lib.rs-0776 */         &self.0
/* FP:lib.rs-0777 */     }
/* FP:lib.rs-0778 */ }
/* FP:lib.rs-0779 */ 
/* FP:lib.rs-0780 */ impl<'ra> fmt::Debug for Module<'ra> {
/* FP:lib.rs-0781 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:lib.rs-0782 */         match self.kind {
/* FP:lib.rs-0783 */             ModuleKind::Block => write!(f, "block"),
/* FP:lib.rs-0784 */             ModuleKind::Def(..) => write!(f, "{:?}", self.res()),
/* FP:lib.rs-0785 */         }
/* FP:lib.rs-0786 */     }
/* FP:lib.rs-0787 */ }
/* FP:lib.rs-0788 */ 
/* FP:lib.rs-0789 */ /// Records a possibly-private value, type, or module definition.
/* FP:lib.rs-0790 */ #[derive(Clone, Copy, Debug)]
/* FP:lib.rs-0791 */ struct NameBindingData<'ra> {
/* FP:lib.rs-0792 */     kind: NameBindingKind<'ra>,
/* FP:lib.rs-0793 */     ambiguity: Option<(NameBinding<'ra>, AmbiguityKind)>,
/* FP:lib.rs-0794 */     /// Produce a warning instead of an error when reporting ambiguities inside this binding.
/* FP:lib.rs-0795 */     /// May apply to indirect ambiguities under imports, so `ambiguity.is_some()` is not required.
/* FP:lib.rs-0796 */     warn_ambiguity: bool,
/* FP:lib.rs-0797 */     expansion: LocalExpnId,
/* FP:lib.rs-0798 */     span: Span,
/* FP:lib.rs-0799 */     vis: Visibility<DefId>,
/* FP:lib.rs-0800 */ }
/* FP:lib.rs-0801 */ 
/* FP:lib.rs-0802 */ /// All name bindings are unique and allocated on a same arena,
/* FP:lib.rs-0803 */ /// so we can use referential equality to compare them.
/* FP:lib.rs-0804 */ type NameBinding<'ra> = Interned<'ra, NameBindingData<'ra>>;
/* FP:lib.rs-0805 */ 
/* FP:lib.rs-0806 */ // Allows us to use Interned without actually enforcing (via Hash/PartialEq/...) uniqueness of the
/* FP:lib.rs-0807 */ // contained data.
/* FP:lib.rs-0808 */ // FIXME: We may wish to actually have at least debug-level assertions that Interned's guarantees
/* FP:lib.rs-0809 */ // are upheld.
/* FP:lib.rs-0810 */ impl std::hash::Hash for NameBindingData<'_> {
/* FP:lib.rs-0811 */     fn hash<H>(&self, _: &mut H)
/* FP:lib.rs-0812 */     where
/* FP:lib.rs-0813 */         H: std::hash::Hasher,
/* FP:lib.rs-0814 */     {
/* FP:lib.rs-0815 */         unreachable!()
/* FP:lib.rs-0816 */     }
/* FP:lib.rs-0817 */ }
/* FP:lib.rs-0818 */ 
/* FP:lib.rs-0819 */ #[derive(Clone, Copy, Debug)]
/* FP:lib.rs-0820 */ enum NameBindingKind<'ra> {
/* FP:lib.rs-0821 */     Res(Res),
/* FP:lib.rs-0822 */     Import { binding: NameBinding<'ra>, import: Import<'ra> },
/* FP:lib.rs-0823 */ }
/* FP:lib.rs-0824 */ 
/* FP:lib.rs-0825 */ impl<'ra> NameBindingKind<'ra> {
/* FP:lib.rs-0826 */     /// Is this a name binding of an import?
/* FP:lib.rs-0827 */     fn is_import(&self) -> bool {
/* FP:lib.rs-0828 */         matches!(*self, NameBindingKind::Import { .. })
/* FP:lib.rs-0829 */     }
/* FP:lib.rs-0830 */ }
/* FP:lib.rs-0831 */ 
/* FP:lib.rs-0832 */ #[derive(Debug)]
/* FP:lib.rs-0833 */ struct PrivacyError<'ra> {
/* FP:lib.rs-0834 */     ident: Ident,
/* FP:lib.rs-0835 */     binding: NameBinding<'ra>,
/* FP:lib.rs-0836 */     dedup_span: Span,
/* FP:lib.rs-0837 */     outermost_res: Option<(Res, Ident)>,
/* FP:lib.rs-0838 */     parent_scope: ParentScope<'ra>,
/* FP:lib.rs-0839 */     /// Is the format `use a::{b,c}`?
/* FP:lib.rs-0840 */     single_nested: bool,
/* FP:lib.rs-0841 */     source: Option<ast::Expr>,
/* FP:lib.rs-0842 */ }
/* FP:lib.rs-0843 */ 
/* FP:lib.rs-0844 */ #[derive(Debug)]
/* FP:lib.rs-0845 */ struct UseError<'a> {
/* FP:lib.rs-0846 */     err: Diag<'a>,
/* FP:lib.rs-0847 */     /// Candidates which user could `use` to access the missing type.
/* FP:lib.rs-0848 */     candidates: Vec<ImportSuggestion>,
/* FP:lib.rs-0849 */     /// The `DefId` of the module to place the use-statements in.
/* FP:lib.rs-0850 */     def_id: DefId,
/* FP:lib.rs-0851 */     /// Whether the diagnostic should say "instead" (as in `consider importing ... instead`).
/* FP:lib.rs-0852 */     instead: bool,
/* FP:lib.rs-0853 */     /// Extra free-form suggestion.
/* FP:lib.rs-0854 */     suggestion: Option<(Span, &'static str, String, Applicability)>,
/* FP:lib.rs-0855 */     /// Path `Segment`s at the place of use that failed. Used for accurate suggestion after telling
/* FP:lib.rs-0856 */     /// the user to import the item directly.
/* FP:lib.rs-0857 */     path: Vec<Segment>,
/* FP:lib.rs-0858 */     /// Whether the expected source is a call
/* FP:lib.rs-0859 */     is_call: bool,
/* FP:lib.rs-0860 */ }
/* FP:lib.rs-0861 */ 
/* FP:lib.rs-0862 */ #[derive(Clone, Copy, PartialEq, Debug)]
/* FP:lib.rs-0863 */ enum AmbiguityKind {
/* FP:lib.rs-0864 */     BuiltinAttr,
/* FP:lib.rs-0865 */     DeriveHelper,
/* FP:lib.rs-0866 */     MacroRulesVsModularized,
/* FP:lib.rs-0867 */     GlobVsOuter,
/* FP:lib.rs-0868 */     GlobVsGlob,
/* FP:lib.rs-0869 */     GlobVsExpanded,
/* FP:lib.rs-0870 */     MoreExpandedVsOuter,
/* FP:lib.rs-0871 */ }
/* FP:lib.rs-0872 */ 
/* FP:lib.rs-0873 */ impl AmbiguityKind {
/* FP:lib.rs-0874 */     fn descr(self) -> &'static str {
/* FP:lib.rs-0875 */         match self {
/* FP:lib.rs-0876 */             AmbiguityKind::BuiltinAttr => "a name conflict with a builtin attribute",
/* FP:lib.rs-0877 */             AmbiguityKind::DeriveHelper => "a name conflict with a derive helper attribute",
/* FP:lib.rs-0878 */             AmbiguityKind::MacroRulesVsModularized => {
/* FP:lib.rs-0879 */                 "a conflict between a `macro_rules` name and a non-`macro_rules` name from another module"
/* FP:lib.rs-0880 */             }
/* FP:lib.rs-0881 */             AmbiguityKind::GlobVsOuter => {
/* FP:lib.rs-0882 */                 "a conflict between a name from a glob import and an outer scope during import or macro resolution"
/* FP:lib.rs-0883 */             }
/* FP:lib.rs-0884 */             AmbiguityKind::GlobVsGlob => "multiple glob imports of a name in the same module",
/* FP:lib.rs-0885 */             AmbiguityKind::GlobVsExpanded => {
/* FP:lib.rs-0886 */                 "a conflict between a name from a glob import and a macro-expanded name in the same module during import or macro resolution"
/* FP:lib.rs-0887 */             }
/* FP:lib.rs-0888 */             AmbiguityKind::MoreExpandedVsOuter => {
/* FP:lib.rs-0889 */                 "a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import or macro resolution"
/* FP:lib.rs-0890 */             }
/* FP:lib.rs-0891 */         }
/* FP:lib.rs-0892 */     }
/* FP:lib.rs-0893 */ }
/* FP:lib.rs-0894 */ 
/* FP:lib.rs-0895 */ /// Miscellaneous bits of metadata for better ambiguity error reporting.
/* FP:lib.rs-0896 */ #[derive(Clone, Copy, PartialEq)]
/* FP:lib.rs-0897 */ enum AmbiguityErrorMisc {
/* FP:lib.rs-0898 */     SuggestCrate,
/* FP:lib.rs-0899 */     SuggestSelf,
/* FP:lib.rs-0900 */     FromPrelude,
/* FP:lib.rs-0901 */     None,
/* FP:lib.rs-0902 */ }
/* FP:lib.rs-0903 */ 
/* FP:lib.rs-0904 */ struct AmbiguityError<'ra> {
/* FP:lib.rs-0905 */     kind: AmbiguityKind,
/* FP:lib.rs-0906 */     ident: Ident,
/* FP:lib.rs-0907 */     b1: NameBinding<'ra>,
/* FP:lib.rs-0908 */     b2: NameBinding<'ra>,
/* FP:lib.rs-0909 */     misc1: AmbiguityErrorMisc,
/* FP:lib.rs-0910 */     misc2: AmbiguityErrorMisc,
/* FP:lib.rs-0911 */     warning: bool,
/* FP:lib.rs-0912 */ }
/* FP:lib.rs-0913 */ 
/* FP:lib.rs-0914 */ impl<'ra> NameBindingData<'ra> {
/* FP:lib.rs-0915 */     fn res(&self) -> Res {
/* FP:lib.rs-0916 */         match self.kind {
/* FP:lib.rs-0917 */             NameBindingKind::Res(res) => res,
/* FP:lib.rs-0918 */             NameBindingKind::Import { binding, .. } => binding.res(),
/* FP:lib.rs-0919 */         }
/* FP:lib.rs-0920 */     }
/* FP:lib.rs-0921 */ 
/* FP:lib.rs-0922 */     fn import_source(&self) -> NameBinding<'ra> {
/* FP:lib.rs-0923 */         match self.kind {
/* FP:lib.rs-0924 */             NameBindingKind::Import { binding, .. } => binding,
/* FP:lib.rs-0925 */             _ => unreachable!(),
/* FP:lib.rs-0926 */         }
/* FP:lib.rs-0927 */     }
/* FP:lib.rs-0928 */ 
/* FP:lib.rs-0929 */     fn is_ambiguity_recursive(&self) -> bool {
/* FP:lib.rs-0930 */         self.ambiguity.is_some()
/* FP:lib.rs-0931 */             || match self.kind {
/* FP:lib.rs-0932 */                 NameBindingKind::Import { binding, .. } => binding.is_ambiguity_recursive(),
/* FP:lib.rs-0933 */                 _ => false,
/* FP:lib.rs-0934 */             }
/* FP:lib.rs-0935 */     }
/* FP:lib.rs-0936 */ 
/* FP:lib.rs-0937 */     fn warn_ambiguity_recursive(&self) -> bool {
/* FP:lib.rs-0938 */         self.warn_ambiguity
/* FP:lib.rs-0939 */             || match self.kind {
/* FP:lib.rs-0940 */                 NameBindingKind::Import { binding, .. } => binding.warn_ambiguity_recursive(),
/* FP:lib.rs-0941 */                 _ => false,
/* FP:lib.rs-0942 */             }
/* FP:lib.rs-0943 */     }
/* FP:lib.rs-0944 */ 
/* FP:lib.rs-0945 */     fn is_possibly_imported_variant(&self) -> bool {
/* FP:lib.rs-0946 */         match self.kind {
/* FP:lib.rs-0947 */             NameBindingKind::Import { binding, .. } => binding.is_possibly_imported_variant(),
/* FP:lib.rs-0948 */             NameBindingKind::Res(Res::Def(
/* FP:lib.rs-0949 */                 DefKind::Variant | DefKind::Ctor(CtorOf::Variant, ..),
/* FP:lib.rs-0950 */                 _,
/* FP:lib.rs-0951 */             )) => true,
/* FP:lib.rs-0952 */             NameBindingKind::Res(..) => false,
/* FP:lib.rs-0953 */         }
/* FP:lib.rs-0954 */     }
/* FP:lib.rs-0955 */ 
/* FP:lib.rs-0956 */     fn is_extern_crate(&self) -> bool {
/* FP:lib.rs-0957 */         match self.kind {
/* FP:lib.rs-0958 */             NameBindingKind::Import { import, .. } => {
/* FP:lib.rs-0959 */                 matches!(import.kind, ImportKind::ExternCrate { .. })
/* FP:lib.rs-0960 */             }
/* FP:lib.rs-0961 */             NameBindingKind::Res(Res::Def(_, def_id)) => def_id.is_crate_root(),
/* FP:lib.rs-0962 */             _ => false,
/* FP:lib.rs-0963 */         }
/* FP:lib.rs-0964 */     }
/* FP:lib.rs-0965 */ 
/* FP:lib.rs-0966 */     fn is_import(&self) -> bool {
/* FP:lib.rs-0967 */         matches!(self.kind, NameBindingKind::Import { .. })
/* FP:lib.rs-0968 */     }
/* FP:lib.rs-0969 */ 
/* FP:lib.rs-0970 */     /// The binding introduced by `#[macro_export] macro_rules` is a public import, but it might
/* FP:lib.rs-0971 */     /// not be perceived as such by users, so treat it as a non-import in some diagnostics.
/* FP:lib.rs-0972 */     fn is_import_user_facing(&self) -> bool {
/* FP:lib.rs-0973 */         matches!(self.kind, NameBindingKind::Import { import, .. }
/* FP:lib.rs-0974 */             if !matches!(import.kind, ImportKind::MacroExport))
/* FP:lib.rs-0975 */     }
/* FP:lib.rs-0976 */ 
/* FP:lib.rs-0977 */     fn is_glob_import(&self) -> bool {
/* FP:lib.rs-0978 */         match self.kind {
/* FP:lib.rs-0979 */             NameBindingKind::Import { import, .. } => import.is_glob(),
/* FP:lib.rs-0980 */             _ => false,
/* FP:lib.rs-0981 */         }
/* FP:lib.rs-0982 */     }
/* FP:lib.rs-0983 */ 
/* FP:lib.rs-0984 */     fn is_assoc_item(&self) -> bool {
/* FP:lib.rs-0985 */         matches!(self.res(), Res::Def(DefKind::AssocConst | DefKind::AssocFn | DefKind::AssocTy, _))
/* FP:lib.rs-0986 */     }
/* FP:lib.rs-0987 */ 
/* FP:lib.rs-0988 */     fn macro_kinds(&self) -> Option<MacroKinds> {
/* FP:lib.rs-0989 */         self.res().macro_kinds()
/* FP:lib.rs-0990 */     }
/* FP:lib.rs-0991 */ 
/* FP:lib.rs-0992 */     // Suppose that we resolved macro invocation with `invoc_parent_expansion` to binding `binding`
/* FP:lib.rs-0993 */     // at some expansion round `max(invoc, binding)` when they both emerged from macros.
/* FP:lib.rs-0994 */     // Then this function returns `true` if `self` may emerge from a macro *after* that
/* FP:lib.rs-0995 */     // in some later round and screw up our previously found resolution.
/* FP:lib.rs-0996 */     // See more detailed explanation in
/* FP:lib.rs-0997 */     // https://github.com/rust-lang/rust/pull/53778#issuecomment-419224049
/* FP:lib.rs-0998 */     fn may_appear_after(
/* FP:lib.rs-0999 */         &self,
/* FP:lib.rs-1000 */         invoc_parent_expansion: LocalExpnId,
/* FP:lib.rs-1001 */         binding: NameBinding<'_>,
/* FP:lib.rs-1002 */     ) -> bool {
/* FP:lib.rs-1003 */         // self > max(invoc, binding) => !(self <= invoc || self <= binding)
/* FP:lib.rs-1004 */         // Expansions are partially ordered, so "may appear after" is an inversion of
/* FP:lib.rs-1005 */         // "certainly appears before or simultaneously" and includes unordered cases.
/* FP:lib.rs-1006 */         let self_parent_expansion = self.expansion;
/* FP:lib.rs-1007 */         let other_parent_expansion = binding.expansion;
/* FP:lib.rs-1008 */         let certainly_before_other_or_simultaneously =
/* FP:lib.rs-1009 */             other_parent_expansion.is_descendant_of(self_parent_expansion);
/* FP:lib.rs-1010 */         let certainly_before_invoc_or_simultaneously =
/* FP:lib.rs-1011 */             invoc_parent_expansion.is_descendant_of(self_parent_expansion);
/* FP:lib.rs-1012 */         !(certainly_before_other_or_simultaneously || certainly_before_invoc_or_simultaneously)
/* FP:lib.rs-1013 */     }
/* FP:lib.rs-1014 */ 
/* FP:lib.rs-1015 */     // Its purpose is to postpone the determination of a single binding because
/* FP:lib.rs-1016 */     // we can't predict whether it will be overwritten by recently expanded macros.
/* FP:lib.rs-1017 */     // FIXME: How can we integrate it with the `update_resolution`?
/* FP:lib.rs-1018 */     fn determined(&self) -> bool {
/* FP:lib.rs-1019 */         match &self.kind {
/* FP:lib.rs-1020 */             NameBindingKind::Import { binding, import, .. } if import.is_glob() => {
/* FP:lib.rs-1021 */                 import.parent_scope.module.unexpanded_invocations.borrow().is_empty()
/* FP:lib.rs-1022 */                     && binding.determined()
/* FP:lib.rs-1023 */             }
/* FP:lib.rs-1024 */             _ => true,
/* FP:lib.rs-1025 */         }
/* FP:lib.rs-1026 */     }
/* FP:lib.rs-1027 */ }
/* FP:lib.rs-1028 */ 
/* FP:lib.rs-1029 */ struct ExternPreludeEntry<'ra> {
/* FP:lib.rs-1030 */     /// Binding from an `extern crate` item.
/* FP:lib.rs-1031 */     /// The boolean flag is true is `item_binding` is non-redundant, happens either when
/* FP:lib.rs-1032 */     /// `flag_binding` is `None`, or when `extern crate` introducing `item_binding` used renaming.
/* FP:lib.rs-1033 */     item_binding: Option<(NameBinding<'ra>, /* introduced by item */ bool)>,
/* FP:lib.rs-1034 */     /// Binding from an `--extern` flag, lazily populated on first use.
/* FP:lib.rs-1035 */     flag_binding: Option<Cell<(PendingBinding<'ra>, /* finalized */ bool)>>,
/* FP:lib.rs-1036 */ }
/* FP:lib.rs-1037 */ 
/* FP:lib.rs-1038 */ impl ExternPreludeEntry<'_> {
/* FP:lib.rs-1039 */     fn introduced_by_item(&self) -> bool {
/* FP:lib.rs-1040 */         matches!(self.item_binding, Some((_, true)))
/* FP:lib.rs-1041 */     }
/* FP:lib.rs-1042 */ 
/* FP:lib.rs-1043 */     fn flag() -> Self {
/* FP:lib.rs-1044 */         ExternPreludeEntry {
/* FP:lib.rs-1045 */             item_binding: None,
/* FP:lib.rs-1046 */             flag_binding: Some(Cell::new((PendingBinding::Pending, false))),
/* FP:lib.rs-1047 */         }
/* FP:lib.rs-1048 */     }
/* FP:lib.rs-1049 */ }
/* FP:lib.rs-1050 */ 
/* FP:lib.rs-1051 */ struct DeriveData {
/* FP:lib.rs-1052 */     resolutions: Vec<DeriveResolution>,
/* FP:lib.rs-1053 */     helper_attrs: Vec<(usize, Ident)>,
/* FP:lib.rs-1054 */     has_derive_copy: bool,
/* FP:lib.rs-1055 */ }
/* FP:lib.rs-1056 */ 
/* FP:lib.rs-1057 */ struct MacroData {
/* FP:lib.rs-1058 */     ext: Arc<SyntaxExtension>,
/* FP:lib.rs-1059 */     nrules: usize,
/* FP:lib.rs-1060 */     macro_rules: bool,
/* FP:lib.rs-1061 */ }
/* FP:lib.rs-1062 */ 
/* FP:lib.rs-1063 */ impl MacroData {
/* FP:lib.rs-1064 */     fn new(ext: Arc<SyntaxExtension>) -> MacroData {
/* FP:lib.rs-1065 */         MacroData { ext, nrules: 0, macro_rules: false }
/* FP:lib.rs-1066 */     }
/* FP:lib.rs-1067 */ }
/* FP:lib.rs-1068 */ 
/* FP:lib.rs-1069 */ pub struct ResolverOutputs {
/* FP:lib.rs-1070 */     pub global_ctxt: ResolverGlobalCtxt,
/* FP:lib.rs-1071 */     pub ast_lowering: ResolverAstLowering,
/* FP:lib.rs-1072 */ }
/* FP:lib.rs-1073 */ 
/* FP:lib.rs-1074 */ /// The main resolver class.
/* FP:lib.rs-1075 */ ///
/* FP:lib.rs-1076 */ /// This is the visitor that walks the whole crate.
/* FP:lib.rs-1077 */ pub struct Resolver<'ra, 'tcx> {
/* FP:lib.rs-1078 */     tcx: TyCtxt<'tcx>,
/* FP:lib.rs-1079 */ 
/* FP:lib.rs-1080 */     /// Item with a given `LocalDefId` was defined during macro expansion with ID `ExpnId`.
/* FP:lib.rs-1081 */     expn_that_defined: UnordMap<LocalDefId, ExpnId>,
/* FP:lib.rs-1082 */ 
/* FP:lib.rs-1083 */     graph_root: Module<'ra>,
/* FP:lib.rs-1084 */ 
/* FP:lib.rs-1085 */     /// Assert that we are in speculative resolution mode.
/* FP:lib.rs-1086 */     assert_speculative: bool,
/* FP:lib.rs-1087 */ 
/* FP:lib.rs-1088 */     prelude: Option<Module<'ra>> = None,
/* FP:lib.rs-1089 */     extern_prelude: FxIndexMap<Macros20NormalizedIdent, ExternPreludeEntry<'ra>>,
/* FP:lib.rs-1090 */ 
/* FP:lib.rs-1091 */     /// N.B., this is used only for better diagnostics, not name resolution itself.
/* FP:lib.rs-1092 */     field_names: LocalDefIdMap<Vec<Ident>>,
/* FP:lib.rs-1093 */     field_defaults: LocalDefIdMap<Vec<Symbol>>,
/* FP:lib.rs-1094 */ 
/* FP:lib.rs-1095 */     /// Span of the privacy modifier in fields of an item `DefId` accessible with dot syntax.
/* FP:lib.rs-1096 */     /// Used for hints during error reporting.
/* FP:lib.rs-1097 */     field_visibility_spans: FxHashMap<DefId, Vec<Span>>,
/* FP:lib.rs-1098 */ 
/* FP:lib.rs-1099 */     /// All imports known to succeed or fail.
/* FP:lib.rs-1100 */     determined_imports: Vec<Import<'ra>> = Vec::new(),
/* FP:lib.rs-1101 */ 
/* FP:lib.rs-1102 */     /// All non-determined imports.
/* FP:lib.rs-1103 */     indeterminate_imports: Vec<Import<'ra>> = Vec::new(),
/* FP:lib.rs-1104 */ 
/* FP:lib.rs-1105 */     // Spans for local variables found during pattern resolution.
/* FP:lib.rs-1106 */     // Used for suggestions during error reporting.
/* FP:lib.rs-1107 */     pat_span_map: NodeMap<Span>,
/* FP:lib.rs-1108 */ 
/* FP:lib.rs-1109 */     /// Resolutions for nodes that have a single resolution.
/* FP:lib.rs-1110 */     partial_res_map: NodeMap<PartialRes>,
/* FP:lib.rs-1111 */     /// Resolutions for import nodes, which have multiple resolutions in different namespaces.
/* FP:lib.rs-1112 */     import_res_map: NodeMap<PerNS<Option<Res>>>,
/* FP:lib.rs-1113 */     /// An import will be inserted into this map if it has been used.
/* FP:lib.rs-1114 */     import_use_map: FxHashMap<Import<'ra>, Used>,
/* FP:lib.rs-1115 */     /// Resolutions for labels (node IDs of their corresponding blocks or loops).
/* FP:lib.rs-1116 */     label_res_map: NodeMap<NodeId>,
/* FP:lib.rs-1117 */     /// Resolutions for lifetimes.
/* FP:lib.rs-1118 */     lifetimes_res_map: NodeMap<LifetimeRes>,
/* FP:lib.rs-1119 */     /// Lifetime parameters that lowering will have to introduce.
/* FP:lib.rs-1120 */     extra_lifetime_params_map: NodeMap<Vec<(Ident, NodeId, LifetimeRes)>>,
/* FP:lib.rs-1121 */ 
/* FP:lib.rs-1122 */     /// `CrateNum` resolutions of `extern crate` items.
/* FP:lib.rs-1123 */     extern_crate_map: UnordMap<LocalDefId, CrateNum>,
/* FP:lib.rs-1124 */     module_children: LocalDefIdMap<Vec<ModChild>>,
/* FP:lib.rs-1125 */     trait_map: NodeMap<Vec<TraitCandidate>>,
/* FP:lib.rs-1126 */ 
/* FP:lib.rs-1127 */     /// A map from nodes to anonymous modules.
/* FP:lib.rs-1128 */     /// Anonymous modules are pseudo-modules that are implicitly created around items
/* FP:lib.rs-1129 */     /// contained within blocks.
/* FP:lib.rs-1130 */     ///
/* FP:lib.rs-1131 */     /// For example, if we have this:
/* FP:lib.rs-1132 */     ///
/* FP:lib.rs-1133 */     ///  fn f() {
/* FP:lib.rs-1134 */     ///      fn g() {
/* FP:lib.rs-1135 */     ///          ...
/* FP:lib.rs-1136 */     ///      }
/* FP:lib.rs-1137 */     ///  }
/* FP:lib.rs-1138 */     ///
/* FP:lib.rs-1139 */     /// There will be an anonymous module created around `g` with the ID of the
/* FP:lib.rs-1140 */     /// entry block for `f`.
/* FP:lib.rs-1141 */     block_map: NodeMap<Module<'ra>>,
/* FP:lib.rs-1142 */     /// A fake module that contains no definition and no prelude. Used so that
/* FP:lib.rs-1143 */     /// some AST passes can generate identifiers that only resolve to local or
/* FP:lib.rs-1144 */     /// lang items.
/* FP:lib.rs-1145 */     empty_module: Module<'ra>,
/* FP:lib.rs-1146 */     /// Eagerly populated map of all local non-block modules.
/* FP:lib.rs-1147 */     local_module_map: FxIndexMap<LocalDefId, Module<'ra>>,
/* FP:lib.rs-1148 */     /// Lazily populated cache of modules loaded from external crates.
/* FP:lib.rs-1149 */     extern_module_map: RefCell<FxIndexMap<DefId, Module<'ra>>>,
/* FP:lib.rs-1150 */     binding_parent_modules: FxHashMap<NameBinding<'ra>, Module<'ra>>,
/* FP:lib.rs-1151 */ 
/* FP:lib.rs-1152 */     /// Maps glob imports to the names of items actually imported.
/* FP:lib.rs-1153 */     glob_map: FxIndexMap<LocalDefId, FxIndexSet<Symbol>>,
/* FP:lib.rs-1154 */     glob_error: Option<ErrorGuaranteed> = None,
/* FP:lib.rs-1155 */     visibilities_for_hashing: Vec<(LocalDefId, Visibility)> = Vec::new(),
/* FP:lib.rs-1156 */     used_imports: FxHashSet<NodeId>,
/* FP:lib.rs-1157 */     maybe_unused_trait_imports: FxIndexSet<LocalDefId>,
/* FP:lib.rs-1158 */ 
/* FP:lib.rs-1159 */     /// Privacy errors are delayed until the end in order to deduplicate them.
/* FP:lib.rs-1160 */     privacy_errors: Vec<PrivacyError<'ra>> = Vec::new(),
/* FP:lib.rs-1161 */     /// Ambiguity errors are delayed for deduplication.
/* FP:lib.rs-1162 */     ambiguity_errors: Vec<AmbiguityError<'ra>> = Vec::new(),
/* FP:lib.rs-1163 */     /// `use` injections are delayed for better placement and deduplication.
/* FP:lib.rs-1164 */     use_injections: Vec<UseError<'tcx>> = Vec::new(),
/* FP:lib.rs-1165 */     /// Crate-local macro expanded `macro_export` referred to by a module-relative path.
/* FP:lib.rs-1166 */     macro_expanded_macro_export_errors: BTreeSet<(Span, Span)> = BTreeSet::new(),
/* FP:lib.rs-1167 */ 
/* FP:lib.rs-1168 */     arenas: &'ra ResolverArenas<'ra>,
/* FP:lib.rs-1169 */     dummy_binding: NameBinding<'ra>,
/* FP:lib.rs-1170 */     builtin_types_bindings: FxHashMap<Symbol, NameBinding<'ra>>,
/* FP:lib.rs-1171 */     builtin_attrs_bindings: FxHashMap<Symbol, NameBinding<'ra>>,
/* FP:lib.rs-1172 */     registered_tool_bindings: FxHashMap<Ident, NameBinding<'ra>>,
/* FP:lib.rs-1173 */     macro_names: FxHashSet<Ident>,
/* FP:lib.rs-1174 */     builtin_macros: FxHashMap<Symbol, SyntaxExtensionKind>,
/* FP:lib.rs-1175 */     registered_tools: &'tcx RegisteredTools,
/* FP:lib.rs-1176 */     macro_use_prelude: FxIndexMap<Symbol, NameBinding<'ra>>,
/* FP:lib.rs-1177 */     /// Eagerly populated map of all local macro definitions.
/* FP:lib.rs-1178 */     local_macro_map: FxHashMap<LocalDefId, &'ra MacroData>,
/* FP:lib.rs-1179 */     /// Lazily populated cache of macro definitions loaded from external crates.
/* FP:lib.rs-1180 */     extern_macro_map: RefCell<FxHashMap<DefId, &'ra MacroData>>,
/* FP:lib.rs-1181 */     dummy_ext_bang: Arc<SyntaxExtension>,
/* FP:lib.rs-1182 */     dummy_ext_derive: Arc<SyntaxExtension>,
/* FP:lib.rs-1183 */     non_macro_attr: &'ra MacroData,
/* FP:lib.rs-1184 */     local_macro_def_scopes: FxHashMap<LocalDefId, Module<'ra>>,
/* FP:lib.rs-1185 */     ast_transform_scopes: FxHashMap<LocalExpnId, Module<'ra>>,
/* FP:lib.rs-1186 */     unused_macros: FxIndexMap<LocalDefId, (NodeId, Ident)>,
/* FP:lib.rs-1187 */     /// A map from the macro to all its potentially unused arms.
/* FP:lib.rs-1188 */     unused_macro_rules: FxIndexMap<NodeId, DenseBitSet<usize>>,
/* FP:lib.rs-1189 */     proc_macro_stubs: FxHashSet<LocalDefId>,
/* FP:lib.rs-1190 */     /// Traces collected during macro resolution and validated when it's complete.
/* FP:lib.rs-1191 */     // FIXME: Remove interior mutability when speculative resolution produces these as outputs.
/* FP:lib.rs-1192 */     single_segment_macro_resolutions:
/* FP:lib.rs-1193 */         RefCell<Vec<(Ident, MacroKind, ParentScope<'ra>, Option<NameBinding<'ra>>, Option<Span>)>>,
/* FP:lib.rs-1194 */     multi_segment_macro_resolutions:
/* FP:lib.rs-1195 */         RefCell<Vec<(Vec<Segment>, Span, MacroKind, ParentScope<'ra>, Option<Res>, Namespace)>>,
/* FP:lib.rs-1196 */     builtin_attrs: Vec<(Ident, ParentScope<'ra>)>,
/* FP:lib.rs-1197 */     /// `derive(Copy)` marks items they are applied to so they are treated specially later.
/* FP:lib.rs-1198 */     /// Derive macros cannot modify the item themselves and have to store the markers in the global
/* FP:lib.rs-1199 */     /// context, so they attach the markers to derive container IDs using this resolver table.
/* FP:lib.rs-1200 */     containers_deriving_copy: FxHashSet<LocalExpnId>,
/* FP:lib.rs-1201 */     /// Parent scopes in which the macros were invoked.
/* FP:lib.rs-1202 */     /// FIXME: `derives` are missing in these parent scopes and need to be taken from elsewhere.
/* FP:lib.rs-1203 */     invocation_parent_scopes: FxHashMap<LocalExpnId, ParentScope<'ra>>,
/* FP:lib.rs-1204 */     /// `macro_rules` scopes *produced* by expanding the macro invocations,
/* FP:lib.rs-1205 */     /// include all the `macro_rules` items and other invocations generated by them.
/* FP:lib.rs-1206 */     output_macro_rules_scopes: FxHashMap<LocalExpnId, MacroRulesScopeRef<'ra>>,
/* FP:lib.rs-1207 */     /// `macro_rules` scopes produced by `macro_rules` item definitions.
/* FP:lib.rs-1208 */     macro_rules_scopes: FxHashMap<LocalDefId, MacroRulesScopeRef<'ra>>,
/* FP:lib.rs-1209 */     /// Helper attributes that are in scope for the given expansion.
/* FP:lib.rs-1210 */     helper_attrs: FxHashMap<LocalExpnId, Vec<(Ident, NameBinding<'ra>)>>,
/* FP:lib.rs-1211 */     /// Ready or in-progress results of resolving paths inside the `#[derive(...)]` attribute
/* FP:lib.rs-1212 */     /// with the given `ExpnId`.
/* FP:lib.rs-1213 */     derive_data: FxHashMap<LocalExpnId, DeriveData>,
/* FP:lib.rs-1214 */ 
/* FP:lib.rs-1215 */     /// Avoid duplicated errors for "name already defined".
/* FP:lib.rs-1216 */     name_already_seen: FxHashMap<Symbol, Span>,
/* FP:lib.rs-1217 */ 
/* FP:lib.rs-1218 */     potentially_unused_imports: Vec<Import<'ra>> = Vec::new(),
/* FP:lib.rs-1219 */ 
/* FP:lib.rs-1220 */     potentially_unnecessary_qualifications: Vec<UnnecessaryQualification<'ra>> = Vec::new(),
/* FP:lib.rs-1221 */ 
/* FP:lib.rs-1222 */     /// Table for mapping struct IDs into struct constructor IDs,
/* FP:lib.rs-1223 */     /// it's not used during normal resolution, only for better error reporting.
/* FP:lib.rs-1224 */     /// Also includes of list of each fields visibility
/* FP:lib.rs-1225 */     struct_constructors: LocalDefIdMap<(Res, Visibility<DefId>, Vec<Visibility<DefId>>)>,
/* FP:lib.rs-1226 */ 
/* FP:lib.rs-1227 */     lint_buffer: LintBuffer,
/* FP:lib.rs-1228 */ 
/* FP:lib.rs-1229 */     next_node_id: NodeId = CRATE_NODE_ID,
/* FP:lib.rs-1230 */ 
/* FP:lib.rs-1231 */     node_id_to_def_id: NodeMap<Feed<'tcx, LocalDefId>>,
/* FP:lib.rs-1232 */ 
/* FP:lib.rs-1233 */     disambiguator: DisambiguatorState,
/* FP:lib.rs-1234 */ 
/* FP:lib.rs-1235 */     /// Indices of unnamed struct or variant fields with unresolved attributes.
/* FP:lib.rs-1236 */     placeholder_field_indices: FxHashMap<NodeId, usize>,
/* FP:lib.rs-1237 */     /// When collecting definitions from an AST fragment produced by a macro invocation `ExpnId`
/* FP:lib.rs-1238 */     /// we know what parent node that fragment should be attached to thanks to this table,
/* FP:lib.rs-1239 */     /// and how the `impl Trait` fragments were introduced.
/* FP:lib.rs-1240 */     invocation_parents: FxHashMap<LocalExpnId, InvocationParent>,
/* FP:lib.rs-1241 */ 
/* FP:lib.rs-1242 */     legacy_const_generic_args: FxHashMap<DefId, Option<Vec<usize>>>,
/* FP:lib.rs-1243 */     /// Amount of lifetime parameters for each item in the crate.
/* FP:lib.rs-1244 */     item_generics_num_lifetimes: FxHashMap<LocalDefId, usize>,
/* FP:lib.rs-1245 */     delegation_fn_sigs: LocalDefIdMap<DelegationFnSig>,
/* FP:lib.rs-1246 */ 
/* FP:lib.rs-1247 */     main_def: Option<MainDefinition> = None,
/* FP:lib.rs-1248 */     trait_impls: FxIndexMap<DefId, Vec<LocalDefId>>,
/* FP:lib.rs-1249 */     /// A list of proc macro LocalDefIds, written out in the order in which
/* FP:lib.rs-1250 */     /// they are declared in the static array generated by proc_macro_harness.
/* FP:lib.rs-1251 */     proc_macros: Vec<LocalDefId> = Vec::new(),
/* FP:lib.rs-1252 */     confused_type_with_std_module: FxIndexMap<Span, Span>,
/* FP:lib.rs-1253 */     /// Whether lifetime elision was successful.
/* FP:lib.rs-1254 */     lifetime_elision_allowed: FxHashSet<NodeId>,
/* FP:lib.rs-1255 */ 
/* FP:lib.rs-1256 */     /// Names of items that were stripped out via cfg with their corresponding cfg meta item.
/* FP:lib.rs-1257 */     stripped_cfg_items: Vec<StrippedCfgItem<NodeId>> = Vec::new(),
/* FP:lib.rs-1258 */ 
/* FP:lib.rs-1259 */     effective_visibilities: EffectiveVisibilities,
/* FP:lib.rs-1260 */     doc_link_resolutions: FxIndexMap<LocalDefId, DocLinkResMap>,
/* FP:lib.rs-1261 */     doc_link_traits_in_scope: FxIndexMap<LocalDefId, Vec<DefId>>,
/* FP:lib.rs-1262 */     all_macro_rules: UnordSet<Symbol>,
/* FP:lib.rs-1263 */ 
/* FP:lib.rs-1264 */     /// Invocation ids of all glob delegations.
/* FP:lib.rs-1265 */     glob_delegation_invoc_ids: FxHashSet<LocalExpnId>,
/* FP:lib.rs-1266 */     /// Analogue of module `unexpanded_invocations` but in trait impls, excluding glob delegations.
/* FP:lib.rs-1267 */     /// Needed because glob delegations wait for all other neighboring macros to expand.
/* FP:lib.rs-1268 */     impl_unexpanded_invocations: FxHashMap<LocalDefId, FxHashSet<LocalExpnId>>,
/* FP:lib.rs-1269 */     /// Simplified analogue of module `resolutions` but in trait impls, excluding glob delegations.
/* FP:lib.rs-1270 */     /// Needed because glob delegations exclude explicitly defined names.
/* FP:lib.rs-1271 */     impl_binding_keys: FxHashMap<LocalDefId, FxHashSet<BindingKey>>,
/* FP:lib.rs-1272 */ 
/* FP:lib.rs-1273 */     /// This is the `Span` where an `extern crate foo;` suggestion would be inserted, if `foo`
/* FP:lib.rs-1274 */     /// could be a crate that wasn't imported. For diagnostics use only.
/* FP:lib.rs-1275 */     current_crate_outer_attr_insert_span: Span,
/* FP:lib.rs-1276 */ 
/* FP:lib.rs-1277 */     mods_with_parse_errors: FxHashSet<DefId>,
/* FP:lib.rs-1278 */ 
/* FP:lib.rs-1279 */     /// Whether `Resolver::register_macros_for_all_crates` has been called once already, as we
/* FP:lib.rs-1280 */     /// don't need to run it more than once.
/* FP:lib.rs-1281 */     all_crate_macros_already_registered: bool = false,
/* FP:lib.rs-1282 */ 
/* FP:lib.rs-1283 */     // Stores pre-expansion and pre-placeholder-fragment-insertion names for `impl Trait` types
/* FP:lib.rs-1284 */     // that were encountered during resolution. These names are used to generate item names
/* FP:lib.rs-1285 */     // for APITs, so we don't want to leak details of resolution into these names.
/* FP:lib.rs-1286 */     impl_trait_names: FxHashMap<NodeId, Symbol>,
/* FP:lib.rs-1287 */ }
/* FP:lib.rs-1288 */ 
/* FP:lib.rs-1289 */ /// This provides memory for the rest of the crate. The `'ra` lifetime that is
/* FP:lib.rs-1290 */ /// used by many types in this crate is an abbreviation of `ResolverArenas`.
/* FP:lib.rs-1291 */ #[derive(Default)]
/* FP:lib.rs-1292 */ pub struct ResolverArenas<'ra> {
/* FP:lib.rs-1293 */     modules: TypedArena<ModuleData<'ra>>,
/* FP:lib.rs-1294 */     local_modules: RefCell<Vec<Module<'ra>>>,
/* FP:lib.rs-1295 */     imports: TypedArena<ImportData<'ra>>,
/* FP:lib.rs-1296 */     name_resolutions: TypedArena<RefCell<NameResolution<'ra>>>,
/* FP:lib.rs-1297 */     ast_paths: TypedArena<ast::Path>,
/* FP:lib.rs-1298 */     macros: TypedArena<MacroData>,
/* FP:lib.rs-1299 */     dropless: DroplessArena,
/* FP:lib.rs-1300 */ }
/* FP:lib.rs-1301 */ 
/* FP:lib.rs-1302 */ impl<'ra> ResolverArenas<'ra> {
/* FP:lib.rs-1303 */     fn new_res_binding(
/* FP:lib.rs-1304 */         &'ra self,
/* FP:lib.rs-1305 */         res: Res,
/* FP:lib.rs-1306 */         vis: Visibility<DefId>,
/* FP:lib.rs-1307 */         span: Span,
/* FP:lib.rs-1308 */         expansion: LocalExpnId,
/* FP:lib.rs-1309 */     ) -> NameBinding<'ra> {
/* FP:lib.rs-1310 */         self.alloc_name_binding(NameBindingData {
/* FP:lib.rs-1311 */             kind: NameBindingKind::Res(res),
/* FP:lib.rs-1312 */             ambiguity: None,
/* FP:lib.rs-1313 */             warn_ambiguity: false,
/* FP:lib.rs-1314 */             vis,
/* FP:lib.rs-1315 */             span,
/* FP:lib.rs-1316 */             expansion,
/* FP:lib.rs-1317 */         })
/* FP:lib.rs-1318 */     }
/* FP:lib.rs-1319 */ 
/* FP:lib.rs-1320 */     fn new_pub_res_binding(
/* FP:lib.rs-1321 */         &'ra self,
/* FP:lib.rs-1322 */         res: Res,
/* FP:lib.rs-1323 */         span: Span,
/* FP:lib.rs-1324 */         expn_id: LocalExpnId,
/* FP:lib.rs-1325 */     ) -> NameBinding<'ra> {
/* FP:lib.rs-1326 */         self.new_res_binding(res, Visibility::Public, span, expn_id)
/* FP:lib.rs-1327 */     }
/* FP:lib.rs-1328 */ 
/* FP:lib.rs-1329 */     fn new_module(
/* FP:lib.rs-1330 */         &'ra self,
/* FP:lib.rs-1331 */         parent: Option<Module<'ra>>,
/* FP:lib.rs-1332 */         kind: ModuleKind,
/* FP:lib.rs-1333 */         expn_id: ExpnId,
/* FP:lib.rs-1334 */         span: Span,
/* FP:lib.rs-1335 */         no_implicit_prelude: bool,
/* FP:lib.rs-1336 */     ) -> Module<'ra> {
/* FP:lib.rs-1337 */         let (def_id, self_binding) = match kind {
/* FP:lib.rs-1338 */             ModuleKind::Def(def_kind, def_id, _) => (
/* FP:lib.rs-1339 */                 Some(def_id),
/* FP:lib.rs-1340 */                 Some(self.new_pub_res_binding(Res::Def(def_kind, def_id), span, LocalExpnId::ROOT)),
/* FP:lib.rs-1341 */             ),
/* FP:lib.rs-1342 */             ModuleKind::Block => (None, None),
/* FP:lib.rs-1343 */         };
/* FP:lib.rs-1344 */         let module = Module(Interned::new_unchecked(self.modules.alloc(ModuleData::new(
/* FP:lib.rs-1345 */             parent,
/* FP:lib.rs-1346 */             kind,
/* FP:lib.rs-1347 */             expn_id,
/* FP:lib.rs-1348 */             span,
/* FP:lib.rs-1349 */             no_implicit_prelude,
/* FP:lib.rs-1350 */             self_binding,
/* FP:lib.rs-1351 */         ))));
/* FP:lib.rs-1352 */         if def_id.is_none_or(|def_id| def_id.is_local()) {
/* FP:lib.rs-1353 */             self.local_modules.borrow_mut().push(module);
/* FP:lib.rs-1354 */         }
/* FP:lib.rs-1355 */         module
/* FP:lib.rs-1356 */     }
/* FP:lib.rs-1357 */     fn local_modules(&'ra self) -> std::cell::Ref<'ra, Vec<Module<'ra>>> {
/* FP:lib.rs-1358 */         self.local_modules.borrow()
/* FP:lib.rs-1359 */     }
/* FP:lib.rs-1360 */     fn alloc_name_binding(&'ra self, name_binding: NameBindingData<'ra>) -> NameBinding<'ra> {
/* FP:lib.rs-1361 */         Interned::new_unchecked(self.dropless.alloc(name_binding))
/* FP:lib.rs-1362 */     }
/* FP:lib.rs-1363 */     fn alloc_import(&'ra self, import: ImportData<'ra>) -> Import<'ra> {
/* FP:lib.rs-1364 */         Interned::new_unchecked(self.imports.alloc(import))
/* FP:lib.rs-1365 */     }
/* FP:lib.rs-1366 */     fn alloc_name_resolution(&'ra self) -> &'ra RefCell<NameResolution<'ra>> {
/* FP:lib.rs-1367 */         self.name_resolutions.alloc(Default::default())
/* FP:lib.rs-1368 */     }
/* FP:lib.rs-1369 */     fn alloc_macro_rules_scope(&'ra self, scope: MacroRulesScope<'ra>) -> MacroRulesScopeRef<'ra> {
/* FP:lib.rs-1370 */         self.dropless.alloc(Cell::new(scope))
/* FP:lib.rs-1371 */     }
/* FP:lib.rs-1372 */     fn alloc_macro_rules_binding(
/* FP:lib.rs-1373 */         &'ra self,
/* FP:lib.rs-1374 */         binding: MacroRulesBinding<'ra>,
/* FP:lib.rs-1375 */     ) -> &'ra MacroRulesBinding<'ra> {
/* FP:lib.rs-1376 */         self.dropless.alloc(binding)
/* FP:lib.rs-1377 */     }
/* FP:lib.rs-1378 */     fn alloc_ast_paths(&'ra self, paths: &[ast::Path]) -> &'ra [ast::Path] {
/* FP:lib.rs-1379 */         self.ast_paths.alloc_from_iter(paths.iter().cloned())
/* FP:lib.rs-1380 */     }
/* FP:lib.rs-1381 */     fn alloc_macro(&'ra self, macro_data: MacroData) -> &'ra MacroData {
/* FP:lib.rs-1382 */         self.macros.alloc(macro_data)
/* FP:lib.rs-1383 */     }
/* FP:lib.rs-1384 */     fn alloc_pattern_spans(&'ra self, spans: impl Iterator<Item = Span>) -> &'ra [Span] {
/* FP:lib.rs-1385 */         self.dropless.alloc_from_iter(spans)
/* FP:lib.rs-1386 */     }
/* FP:lib.rs-1387 */ }
/* FP:lib.rs-1388 */ 
/* FP:lib.rs-1389 */ impl<'ra, 'tcx> AsMut<Resolver<'ra, 'tcx>> for Resolver<'ra, 'tcx> {
/* FP:lib.rs-1390 */     fn as_mut(&mut self) -> &mut Resolver<'ra, 'tcx> {
/* FP:lib.rs-1391 */         self
/* FP:lib.rs-1392 */     }
/* FP:lib.rs-1393 */ }
/* FP:lib.rs-1394 */ 
/* FP:lib.rs-1395 */ impl<'ra, 'tcx> AsRef<Resolver<'ra, 'tcx>> for Resolver<'ra, 'tcx> {
/* FP:lib.rs-1396 */     fn as_ref(&self) -> &Resolver<'ra, 'tcx> {
/* FP:lib.rs-1397 */         self
/* FP:lib.rs-1398 */     }
/* FP:lib.rs-1399 */ }
/* FP:lib.rs-1400 */ 
/* FP:lib.rs-1401 */ impl<'tcx> Resolver<'_, 'tcx> {
/* FP:lib.rs-1402 */     fn opt_local_def_id(&self, node: NodeId) -> Option<LocalDefId> {
/* FP:lib.rs-1403 */         self.opt_feed(node).map(|f| f.key())
/* FP:lib.rs-1404 */     }
/* FP:lib.rs-1405 */ 
/* FP:lib.rs-1406 */     fn local_def_id(&self, node: NodeId) -> LocalDefId {
/* FP:lib.rs-1407 */         self.feed(node).key()
/* FP:lib.rs-1408 */     }
/* FP:lib.rs-1409 */ 
/* FP:lib.rs-1410 */     fn opt_feed(&self, node: NodeId) -> Option<Feed<'tcx, LocalDefId>> {
/* FP:lib.rs-1411 */         self.node_id_to_def_id.get(&node).copied()
/* FP:lib.rs-1412 */     }
/* FP:lib.rs-1413 */ 
/* FP:lib.rs-1414 */     fn feed(&self, node: NodeId) -> Feed<'tcx, LocalDefId> {
/* FP:lib.rs-1415 */         self.opt_feed(node).unwrap_or_else(|| panic!("no entry for node id: `{node:?}`"))
/* FP:lib.rs-1416 */     }
/* FP:lib.rs-1417 */ 
/* FP:lib.rs-1418 */     fn local_def_kind(&self, node: NodeId) -> DefKind {
/* FP:lib.rs-1419 */         self.tcx.def_kind(self.local_def_id(node))
/* FP:lib.rs-1420 */     }
/* FP:lib.rs-1421 */ 
/* FP:lib.rs-1422 */     /// Adds a definition with a parent definition.
/* FP:lib.rs-1423 */     fn create_def(
/* FP:lib.rs-1424 */         &mut self,
/* FP:lib.rs-1425 */         parent: LocalDefId,
/* FP:lib.rs-1426 */         node_id: ast::NodeId,
/* FP:lib.rs-1427 */         name: Option<Symbol>,
/* FP:lib.rs-1428 */         def_kind: DefKind,
/* FP:lib.rs-1429 */         expn_id: ExpnId,
/* FP:lib.rs-1430 */         span: Span,
/* FP:lib.rs-1431 */     ) -> TyCtxtFeed<'tcx, LocalDefId> {
/* FP:lib.rs-1432 */         assert!(
/* FP:lib.rs-1433 */             !self.node_id_to_def_id.contains_key(&node_id),
/* FP:lib.rs-1434 */             "adding a def for node-id {:?}, name {:?}, data {:?} but a previous def exists: {:?}",
/* FP:lib.rs-1435 */             node_id,
/* FP:lib.rs-1436 */             name,
/* FP:lib.rs-1437 */             def_kind,
/* FP:lib.rs-1438 */             self.tcx.definitions_untracked().def_key(self.node_id_to_def_id[&node_id].key()),
/* FP:lib.rs-1439 */         );
/* FP:lib.rs-1440 */ 
/* FP:lib.rs-1441 */         // FIXME: remove `def_span` body, pass in the right spans here and call `tcx.at().create_def()`
/* FP:lib.rs-1442 */         let feed = self.tcx.create_def(parent, name, def_kind, None, &mut self.disambiguator);
/* FP:lib.rs-1443 */         let def_id = feed.def_id();
/* FP:lib.rs-1444 */ 
/* FP:lib.rs-1445 */         // Create the definition.
/* FP:lib.rs-1446 */         if expn_id != ExpnId::root() {
/* FP:lib.rs-1447 */             self.expn_that_defined.insert(def_id, expn_id);
/* FP:lib.rs-1448 */         }
/* FP:lib.rs-1449 */ 
/* FP:lib.rs-1450 */         // A relative span's parent must be an absolute span.
/* FP:lib.rs-1451 */         debug_assert_eq!(span.data_untracked().parent, None);
/* FP:lib.rs-1452 */         let _id = self.tcx.untracked().source_span.push(span);
/* FP:lib.rs-1453 */         debug_assert_eq!(_id, def_id);
/* FP:lib.rs-1454 */ 
/* FP:lib.rs-1455 */         // Some things for which we allocate `LocalDefId`s don't correspond to
/* FP:lib.rs-1456 */         // anything in the AST, so they don't have a `NodeId`. For these cases
/* FP:lib.rs-1457 */         // we don't need a mapping from `NodeId` to `LocalDefId`.
/* FP:lib.rs-1458 */         if node_id != ast::DUMMY_NODE_ID {
/* FP:lib.rs-1459 */             debug!("create_def: def_id_to_node_id[{:?}] <-> {:?}", def_id, node_id);
/* FP:lib.rs-1460 */             self.node_id_to_def_id.insert(node_id, feed.downgrade());
/* FP:lib.rs-1461 */         }
/* FP:lib.rs-1462 */ 
/* FP:lib.rs-1463 */         feed
/* FP:lib.rs-1464 */     }
/* FP:lib.rs-1465 */ 
/* FP:lib.rs-1466 */     fn item_generics_num_lifetimes(&self, def_id: DefId) -> usize {
/* FP:lib.rs-1467 */         if let Some(def_id) = def_id.as_local() {
/* FP:lib.rs-1468 */             self.item_generics_num_lifetimes[&def_id]
/* FP:lib.rs-1469 */         } else {
/* FP:lib.rs-1470 */             self.tcx.generics_of(def_id).own_counts().lifetimes
/* FP:lib.rs-1471 */         }
/* FP:lib.rs-1472 */     }
/* FP:lib.rs-1473 */ 
/* FP:lib.rs-1474 */     pub fn tcx(&self) -> TyCtxt<'tcx> {
/* FP:lib.rs-1475 */         self.tcx
/* FP:lib.rs-1476 */     }
/* FP:lib.rs-1477 */ 
/* FP:lib.rs-1478 */     /// This function is very slow, as it iterates over the entire
/* FP:lib.rs-1479 */     /// [Resolver::node_id_to_def_id] map just to find the [NodeId]
/* FP:lib.rs-1480 */     /// that corresponds to the given [LocalDefId]. Only use this in
/* FP:lib.rs-1481 */     /// diagnostics code paths.
/* FP:lib.rs-1482 */     fn def_id_to_node_id(&self, def_id: LocalDefId) -> NodeId {
/* FP:lib.rs-1483 */         self.node_id_to_def_id
/* FP:lib.rs-1484 */             .items()
/* FP:lib.rs-1485 */             .filter(|(_, v)| v.key() == def_id)
/* FP:lib.rs-1486 */             .map(|(k, _)| *k)
/* FP:lib.rs-1487 */             .get_only()
/* FP:lib.rs-1488 */             .unwrap()
/* FP:lib.rs-1489 */     }
/* FP:lib.rs-1490 */ }
/* FP:lib.rs-1491 */ 
/* FP:lib.rs-1492 */ impl<'ra, 'tcx> Resolver<'ra, 'tcx> {
/* FP:lib.rs-1493 */     pub fn new(
/* FP:lib.rs-1494 */         tcx: TyCtxt<'tcx>,
/* FP:lib.rs-1495 */         attrs: &[ast::Attribute],
/* FP:lib.rs-1496 */         crate_span: Span,
/* FP:lib.rs-1497 */         current_crate_outer_attr_insert_span: Span,
/* FP:lib.rs-1498 */         arenas: &'ra ResolverArenas<'ra>,
/* FP:lib.rs-1499 */     ) -> Resolver<'ra, 'tcx> {
/* FP:lib.rs-1500 */         let root_def_id = CRATE_DEF_ID.to_def_id();
/* FP:lib.rs-1501 */         let mut local_module_map = FxIndexMap::default();
/* FP:lib.rs-1502 */         let graph_root = arenas.new_module(
/* FP:lib.rs-1503 */             None,
/* FP:lib.rs-1504 */             ModuleKind::Def(DefKind::Mod, root_def_id, None),
/* FP:lib.rs-1505 */             ExpnId::root(),
/* FP:lib.rs-1506 */             crate_span,
/* FP:lib.rs-1507 */             attr::contains_name(attrs, sym::no_implicit_prelude),
/* FP:lib.rs-1508 */         );
/* FP:lib.rs-1509 */         local_module_map.insert(CRATE_DEF_ID, graph_root);
/* FP:lib.rs-1510 */         let empty_module = arenas.new_module(
/* FP:lib.rs-1511 */             None,
/* FP:lib.rs-1512 */             ModuleKind::Def(DefKind::Mod, root_def_id, None),
/* FP:lib.rs-1513 */             ExpnId::root(),
/* FP:lib.rs-1514 */             DUMMY_SP,
/* FP:lib.rs-1515 */             true,
/* FP:lib.rs-1516 */         );
/* FP:lib.rs-1517 */ 
/* FP:lib.rs-1518 */         let mut node_id_to_def_id = NodeMap::default();
/* FP:lib.rs-1519 */         let crate_feed = tcx.create_local_crate_def_id(crate_span);
/* FP:lib.rs-1520 */ 
/* FP:lib.rs-1521 */         crate_feed.def_kind(DefKind::Mod);
/* FP:lib.rs-1522 */         let crate_feed = crate_feed.downgrade();
/* FP:lib.rs-1523 */         node_id_to_def_id.insert(CRATE_NODE_ID, crate_feed);
/* FP:lib.rs-1524 */ 
/* FP:lib.rs-1525 */         let mut invocation_parents = FxHashMap::default();
/* FP:lib.rs-1526 */         invocation_parents.insert(LocalExpnId::ROOT, InvocationParent::ROOT);
/* FP:lib.rs-1527 */ 
/* FP:lib.rs-1528 */         let mut extern_prelude: FxIndexMap<_, _> = tcx
/* FP:lib.rs-1529 */             .sess
/* FP:lib.rs-1530 */             .opts
/* FP:lib.rs-1531 */             .externs
/* FP:lib.rs-1532 */             .iter()
/* FP:lib.rs-1533 */             .filter_map(|(name, entry)| {
/* FP:lib.rs-1534 */                 // Make sure `self`, `super`, `_` etc do not get into extern prelude.
/* FP:lib.rs-1535 */                 // FIXME: reject `--extern self` and similar in option parsing instead.
/* FP:lib.rs-1536 */                 if entry.add_prelude
/* FP:lib.rs-1537 */                     && let name = Symbol::intern(name)
/* FP:lib.rs-1538 */                     && name.can_be_raw()
/* FP:lib.rs-1539 */                 {
/* FP:lib.rs-1540 */                     let ident = Macros20NormalizedIdent::with_dummy_span(name);
/* FP:lib.rs-1541 */                     Some((ident, ExternPreludeEntry::flag()))
/* FP:lib.rs-1542 */                 } else {
/* FP:lib.rs-1543 */                     None
/* FP:lib.rs-1544 */                 }
/* FP:lib.rs-1545 */             })
/* FP:lib.rs-1546 */             .collect();
/* FP:lib.rs-1547 */ 
/* FP:lib.rs-1548 */         if !attr::contains_name(attrs, sym::no_core) {
/* FP:lib.rs-1549 */             let ident = Macros20NormalizedIdent::with_dummy_span(sym::core);
/* FP:lib.rs-1550 */             extern_prelude.insert(ident, ExternPreludeEntry::flag());
/* FP:lib.rs-1551 */             if !attr::contains_name(attrs, sym::no_std) {
/* FP:lib.rs-1552 */                 let ident = Macros20NormalizedIdent::with_dummy_span(sym::std);
/* FP:lib.rs-1553 */                 extern_prelude.insert(ident, ExternPreludeEntry::flag());
/* FP:lib.rs-1554 */             }
/* FP:lib.rs-1555 */         }
/* FP:lib.rs-1556 */ 
/* FP:lib.rs-1557 */         let registered_tools = tcx.registered_tools(());
/* FP:lib.rs-1558 */         let edition = tcx.sess.edition();
/* FP:lib.rs-1559 */ 
/* FP:lib.rs-1560 */         let mut resolver = Resolver {
/* FP:lib.rs-1561 */             tcx,
/* FP:lib.rs-1562 */ 
/* FP:lib.rs-1563 */             expn_that_defined: Default::default(),
/* FP:lib.rs-1564 */ 
/* FP:lib.rs-1565 */             // The outermost module has def ID 0; this is not reflected in the
/* FP:lib.rs-1566 */             // AST.
/* FP:lib.rs-1567 */             graph_root,
/* FP:lib.rs-1568 */             assert_speculative: false, // Only set/cleared in Resolver::resolve_imports for now
/* FP:lib.rs-1569 */             prelude: None,
/* FP:lib.rs-1570 */             extern_prelude,
/* FP:lib.rs-1571 */ 
/* FP:lib.rs-1572 */             field_names: Default::default(),
/* FP:lib.rs-1573 */             field_defaults: Default::default(),
/* FP:lib.rs-1574 */             field_visibility_spans: FxHashMap::default(),
/* FP:lib.rs-1575 */ 
/* FP:lib.rs-1576 */             pat_span_map: Default::default(),
/* FP:lib.rs-1577 */             partial_res_map: Default::default(),
/* FP:lib.rs-1578 */             import_res_map: Default::default(),
/* FP:lib.rs-1579 */             import_use_map: Default::default(),
/* FP:lib.rs-1580 */             label_res_map: Default::default(),
/* FP:lib.rs-1581 */             lifetimes_res_map: Default::default(),
/* FP:lib.rs-1582 */             extra_lifetime_params_map: Default::default(),
/* FP:lib.rs-1583 */             extern_crate_map: Default::default(),
/* FP:lib.rs-1584 */             module_children: Default::default(),
/* FP:lib.rs-1585 */             trait_map: NodeMap::default(),
/* FP:lib.rs-1586 */             empty_module,
/* FP:lib.rs-1587 */             local_module_map,
/* FP:lib.rs-1588 */             extern_module_map: Default::default(),
/* FP:lib.rs-1589 */             block_map: Default::default(),
/* FP:lib.rs-1590 */             binding_parent_modules: FxHashMap::default(),
/* FP:lib.rs-1591 */             ast_transform_scopes: FxHashMap::default(),
/* FP:lib.rs-1592 */ 
/* FP:lib.rs-1593 */             glob_map: Default::default(),
/* FP:lib.rs-1594 */             used_imports: FxHashSet::default(),
/* FP:lib.rs-1595 */             maybe_unused_trait_imports: Default::default(),
/* FP:lib.rs-1596 */ 
/* FP:lib.rs-1597 */             arenas,
/* FP:lib.rs-1598 */             dummy_binding: arenas.new_pub_res_binding(Res::Err, DUMMY_SP, LocalExpnId::ROOT),
/* FP:lib.rs-1599 */             builtin_types_bindings: PrimTy::ALL
/* FP:lib.rs-1600 */                 .iter()
/* FP:lib.rs-1601 */                 .map(|prim_ty| {
/* FP:lib.rs-1602 */                     let res = Res::PrimTy(*prim_ty);
/* FP:lib.rs-1603 */                     let binding = arenas.new_pub_res_binding(res, DUMMY_SP, LocalExpnId::ROOT);
/* FP:lib.rs-1604 */                     (prim_ty.name(), binding)
/* FP:lib.rs-1605 */                 })
/* FP:lib.rs-1606 */                 .collect(),
/* FP:lib.rs-1607 */             builtin_attrs_bindings: BUILTIN_ATTRIBUTES
/* FP:lib.rs-1608 */                 .iter()
/* FP:lib.rs-1609 */                 .map(|builtin_attr| {
/* FP:lib.rs-1610 */                     let res = Res::NonMacroAttr(NonMacroAttrKind::Builtin(builtin_attr.name));
/* FP:lib.rs-1611 */                     let binding = arenas.new_pub_res_binding(res, DUMMY_SP, LocalExpnId::ROOT);
/* FP:lib.rs-1612 */                     (builtin_attr.name, binding)
/* FP:lib.rs-1613 */                 })
/* FP:lib.rs-1614 */                 .collect(),
/* FP:lib.rs-1615 */             registered_tool_bindings: registered_tools
/* FP:lib.rs-1616 */                 .iter()
/* FP:lib.rs-1617 */                 .map(|ident| {
/* FP:lib.rs-1618 */                     let res = Res::ToolMod;
/* FP:lib.rs-1619 */                     let binding = arenas.new_pub_res_binding(res, ident.span, LocalExpnId::ROOT);
/* FP:lib.rs-1620 */                     (*ident, binding)
/* FP:lib.rs-1621 */                 })
/* FP:lib.rs-1622 */                 .collect(),
/* FP:lib.rs-1623 */             macro_names: FxHashSet::default(),
/* FP:lib.rs-1624 */             builtin_macros: Default::default(),
/* FP:lib.rs-1625 */             registered_tools,
/* FP:lib.rs-1626 */             macro_use_prelude: Default::default(),
/* FP:lib.rs-1627 */             local_macro_map: Default::default(),
/* FP:lib.rs-1628 */             extern_macro_map: Default::default(),
/* FP:lib.rs-1629 */             dummy_ext_bang: Arc::new(SyntaxExtension::dummy_bang(edition)),
/* FP:lib.rs-1630 */             dummy_ext_derive: Arc::new(SyntaxExtension::dummy_derive(edition)),
/* FP:lib.rs-1631 */             non_macro_attr: arenas
/* FP:lib.rs-1632 */                 .alloc_macro(MacroData::new(Arc::new(SyntaxExtension::non_macro_attr(edition)))),
/* FP:lib.rs-1633 */             invocation_parent_scopes: Default::default(),
/* FP:lib.rs-1634 */             output_macro_rules_scopes: Default::default(),
/* FP:lib.rs-1635 */             macro_rules_scopes: Default::default(),
/* FP:lib.rs-1636 */             helper_attrs: Default::default(),
/* FP:lib.rs-1637 */             derive_data: Default::default(),
/* FP:lib.rs-1638 */             local_macro_def_scopes: FxHashMap::default(),
/* FP:lib.rs-1639 */             name_already_seen: FxHashMap::default(),
/* FP:lib.rs-1640 */             struct_constructors: Default::default(),
/* FP:lib.rs-1641 */             unused_macros: Default::default(),
/* FP:lib.rs-1642 */             unused_macro_rules: Default::default(),
/* FP:lib.rs-1643 */             proc_macro_stubs: Default::default(),
/* FP:lib.rs-1644 */             single_segment_macro_resolutions: Default::default(),
/* FP:lib.rs-1645 */             multi_segment_macro_resolutions: Default::default(),
/* FP:lib.rs-1646 */             builtin_attrs: Default::default(),
/* FP:lib.rs-1647 */             containers_deriving_copy: Default::default(),
/* FP:lib.rs-1648 */             lint_buffer: LintBuffer::default(),
/* FP:lib.rs-1649 */             node_id_to_def_id,
/* FP:lib.rs-1650 */             disambiguator: DisambiguatorState::new(),
/* FP:lib.rs-1651 */             placeholder_field_indices: Default::default(),
/* FP:lib.rs-1652 */             invocation_parents,
/* FP:lib.rs-1653 */             legacy_const_generic_args: Default::default(),
/* FP:lib.rs-1654 */             item_generics_num_lifetimes: Default::default(),
/* FP:lib.rs-1655 */             trait_impls: Default::default(),
/* FP:lib.rs-1656 */             confused_type_with_std_module: Default::default(),
/* FP:lib.rs-1657 */             lifetime_elision_allowed: Default::default(),
/* FP:lib.rs-1658 */             stripped_cfg_items: Default::default(),
/* FP:lib.rs-1659 */             effective_visibilities: Default::default(),
/* FP:lib.rs-1660 */             doc_link_resolutions: Default::default(),
/* FP:lib.rs-1661 */             doc_link_traits_in_scope: Default::default(),
/* FP:lib.rs-1662 */             all_macro_rules: Default::default(),
/* FP:lib.rs-1663 */             delegation_fn_sigs: Default::default(),
/* FP:lib.rs-1664 */             glob_delegation_invoc_ids: Default::default(),
/* FP:lib.rs-1665 */             impl_unexpanded_invocations: Default::default(),
/* FP:lib.rs-1666 */             impl_binding_keys: Default::default(),
/* FP:lib.rs-1667 */             current_crate_outer_attr_insert_span,
/* FP:lib.rs-1668 */             mods_with_parse_errors: Default::default(),
/* FP:lib.rs-1669 */             impl_trait_names: Default::default(),
/* FP:lib.rs-1670 */             ..
/* FP:lib.rs-1671 */         };
/* FP:lib.rs-1672 */ 
/* FP:lib.rs-1673 */         let root_parent_scope = ParentScope::module(graph_root, resolver.arenas);
/* FP:lib.rs-1674 */         resolver.invocation_parent_scopes.insert(LocalExpnId::ROOT, root_parent_scope);
/* FP:lib.rs-1675 */         resolver.feed_visibility(crate_feed, Visibility::Public);
/* FP:lib.rs-1676 */ 
/* FP:lib.rs-1677 */         resolver
/* FP:lib.rs-1678 */     }
/* FP:lib.rs-1679 */ 
/* FP:lib.rs-1680 */     fn new_local_module(
/* FP:lib.rs-1681 */         &mut self,
/* FP:lib.rs-1682 */         parent: Option<Module<'ra>>,
/* FP:lib.rs-1683 */         kind: ModuleKind,
/* FP:lib.rs-1684 */         expn_id: ExpnId,
/* FP:lib.rs-1685 */         span: Span,
/* FP:lib.rs-1686 */         no_implicit_prelude: bool,
/* FP:lib.rs-1687 */     ) -> Module<'ra> {
/* FP:lib.rs-1688 */         let module = self.arenas.new_module(parent, kind, expn_id, span, no_implicit_prelude);
/* FP:lib.rs-1689 */         if let Some(def_id) = module.opt_def_id() {
/* FP:lib.rs-1690 */             self.local_module_map.insert(def_id.expect_local(), module);
/* FP:lib.rs-1691 */         }
/* FP:lib.rs-1692 */         module
/* FP:lib.rs-1693 */     }
/* FP:lib.rs-1694 */ 
/* FP:lib.rs-1695 */     fn new_extern_module(
/* FP:lib.rs-1696 */         &self,
/* FP:lib.rs-1697 */         parent: Option<Module<'ra>>,
/* FP:lib.rs-1698 */         kind: ModuleKind,
/* FP:lib.rs-1699 */         expn_id: ExpnId,
/* FP:lib.rs-1700 */         span: Span,
/* FP:lib.rs-1701 */         no_implicit_prelude: bool,
/* FP:lib.rs-1702 */     ) -> Module<'ra> {
/* FP:lib.rs-1703 */         let module = self.arenas.new_module(parent, kind, expn_id, span, no_implicit_prelude);
/* FP:lib.rs-1704 */         self.extern_module_map.borrow_mut().insert(module.def_id(), module);
/* FP:lib.rs-1705 */         module
/* FP:lib.rs-1706 */     }
/* FP:lib.rs-1707 */ 
/* FP:lib.rs-1708 */     fn new_local_macro(&mut self, def_id: LocalDefId, macro_data: MacroData) -> &'ra MacroData {
/* FP:lib.rs-1709 */         let mac = self.arenas.alloc_macro(macro_data);
/* FP:lib.rs-1710 */         self.local_macro_map.insert(def_id, mac);
/* FP:lib.rs-1711 */         mac
/* FP:lib.rs-1712 */     }
/* FP:lib.rs-1713 */ 
/* FP:lib.rs-1714 */     fn next_node_id(&mut self) -> NodeId {
/* FP:lib.rs-1715 */         let start = self.next_node_id;
/* FP:lib.rs-1716 */         let next = start.as_u32().checked_add(1).expect("input too large; ran out of NodeIds");
/* FP:lib.rs-1717 */         self.next_node_id = ast::NodeId::from_u32(next);
/* FP:lib.rs-1718 */         start
/* FP:lib.rs-1719 */     }
/* FP:lib.rs-1720 */ 
/* FP:lib.rs-1721 */     fn next_node_ids(&mut self, count: usize) -> std::ops::Range<NodeId> {
/* FP:lib.rs-1722 */         let start = self.next_node_id;
/* FP:lib.rs-1723 */         let end = start.as_usize().checked_add(count).expect("input too large; ran out of NodeIds");
/* FP:lib.rs-1724 */         self.next_node_id = ast::NodeId::from_usize(end);
/* FP:lib.rs-1725 */         start..self.next_node_id
/* FP:lib.rs-1726 */     }
/* FP:lib.rs-1727 */ 
/* FP:lib.rs-1728 */     pub fn lint_buffer(&mut self) -> &mut LintBuffer {
/* FP:lib.rs-1729 */         &mut self.lint_buffer
/* FP:lib.rs-1730 */     }
/* FP:lib.rs-1731 */ 
/* FP:lib.rs-1732 */     pub fn arenas() -> ResolverArenas<'ra> {
/* FP:lib.rs-1733 */         Default::default()
/* FP:lib.rs-1734 */     }
/* FP:lib.rs-1735 */ 
/* FP:lib.rs-1736 */     fn feed_visibility(&mut self, feed: Feed<'tcx, LocalDefId>, vis: Visibility) {
/* FP:lib.rs-1737 */         let feed = feed.upgrade(self.tcx);
/* FP:lib.rs-1738 */         feed.visibility(vis.to_def_id());
/* FP:lib.rs-1739 */         self.visibilities_for_hashing.push((feed.def_id(), vis));
/* FP:lib.rs-1740 */     }
/* FP:lib.rs-1741 */ 
/* FP:lib.rs-1742 */     pub fn into_outputs(self) -> ResolverOutputs {
/* FP:lib.rs-1743 */         let proc_macros = self.proc_macros;
/* FP:lib.rs-1744 */         let expn_that_defined = self.expn_that_defined;
/* FP:lib.rs-1745 */         let extern_crate_map = self.extern_crate_map;
/* FP:lib.rs-1746 */         let maybe_unused_trait_imports = self.maybe_unused_trait_imports;
/* FP:lib.rs-1747 */         let glob_map = self.glob_map;
/* FP:lib.rs-1748 */         let main_def = self.main_def;
/* FP:lib.rs-1749 */         let confused_type_with_std_module = self.confused_type_with_std_module;
/* FP:lib.rs-1750 */         let effective_visibilities = self.effective_visibilities;
/* FP:lib.rs-1751 */ 
/* FP:lib.rs-1752 */         let stripped_cfg_items = self
/* FP:lib.rs-1753 */             .stripped_cfg_items
/* FP:lib.rs-1754 */             .into_iter()
/* FP:lib.rs-1755 */             .filter_map(|item| {
/* FP:lib.rs-1756 */                 let parent_module =
/* FP:lib.rs-1757 */                     self.node_id_to_def_id.get(&item.parent_module)?.key().to_def_id();
/* FP:lib.rs-1758 */                 Some(StrippedCfgItem { parent_module, ident: item.ident, cfg: item.cfg })
/* FP:lib.rs-1759 */             })
/* FP:lib.rs-1760 */             .collect();
/* FP:lib.rs-1761 */ 
/* FP:lib.rs-1762 */         let global_ctxt = ResolverGlobalCtxt {
/* FP:lib.rs-1763 */             expn_that_defined,
/* FP:lib.rs-1764 */             visibilities_for_hashing: self.visibilities_for_hashing,
/* FP:lib.rs-1765 */             effective_visibilities,
/* FP:lib.rs-1766 */             extern_crate_map,
/* FP:lib.rs-1767 */             module_children: self.module_children,
/* FP:lib.rs-1768 */             glob_map,
/* FP:lib.rs-1769 */             maybe_unused_trait_imports,
/* FP:lib.rs-1770 */             main_def,
/* FP:lib.rs-1771 */             trait_impls: self.trait_impls,
/* FP:lib.rs-1772 */             proc_macros,
/* FP:lib.rs-1773 */             confused_type_with_std_module,
/* FP:lib.rs-1774 */             doc_link_resolutions: self.doc_link_resolutions,
/* FP:lib.rs-1775 */             doc_link_traits_in_scope: self.doc_link_traits_in_scope,
/* FP:lib.rs-1776 */             all_macro_rules: self.all_macro_rules,
/* FP:lib.rs-1777 */             stripped_cfg_items,
/* FP:lib.rs-1778 */         };
/* FP:lib.rs-1779 */         let ast_lowering = ty::ResolverAstLowering {
/* FP:lib.rs-1780 */             legacy_const_generic_args: self.legacy_const_generic_args,
/* FP:lib.rs-1781 */             partial_res_map: self.partial_res_map,
/* FP:lib.rs-1782 */             import_res_map: self.import_res_map,
/* FP:lib.rs-1783 */             label_res_map: self.label_res_map,
/* FP:lib.rs-1784 */             lifetimes_res_map: self.lifetimes_res_map,
/* FP:lib.rs-1785 */             extra_lifetime_params_map: self.extra_lifetime_params_map,
/* FP:lib.rs-1786 */             next_node_id: self.next_node_id,
/* FP:lib.rs-1787 */             node_id_to_def_id: self
/* FP:lib.rs-1788 */                 .node_id_to_def_id
/* FP:lib.rs-1789 */                 .into_items()
/* FP:lib.rs-1790 */                 .map(|(k, f)| (k, f.key()))
/* FP:lib.rs-1791 */                 .collect(),
/* FP:lib.rs-1792 */             disambiguator: self.disambiguator,
/* FP:lib.rs-1793 */             trait_map: self.trait_map,
/* FP:lib.rs-1794 */             lifetime_elision_allowed: self.lifetime_elision_allowed,
/* FP:lib.rs-1795 */             lint_buffer: Steal::new(self.lint_buffer),
/* FP:lib.rs-1796 */             delegation_fn_sigs: self.delegation_fn_sigs,
/* FP:lib.rs-1797 */         };
/* FP:lib.rs-1798 */         ResolverOutputs { global_ctxt, ast_lowering }
/* FP:lib.rs-1799 */     }
/* FP:lib.rs-1800 */ 
/* FP:lib.rs-1801 */     fn create_stable_hashing_context(&self) -> StableHashingContext<'_> {
/* FP:lib.rs-1802 */         StableHashingContext::new(self.tcx.sess, self.tcx.untracked())
/* FP:lib.rs-1803 */     }
/* FP:lib.rs-1804 */ 
/* FP:lib.rs-1805 */     fn cstore(&self) -> FreezeReadGuard<'_, CStore> {
/* FP:lib.rs-1806 */         CStore::from_tcx(self.tcx)
/* FP:lib.rs-1807 */     }
/* FP:lib.rs-1808 */ 
/* FP:lib.rs-1809 */     fn cstore_mut(&self) -> FreezeWriteGuard<'_, CStore> {
/* FP:lib.rs-1810 */         CStore::from_tcx_mut(self.tcx)
/* FP:lib.rs-1811 */     }
/* FP:lib.rs-1812 */ 
/* FP:lib.rs-1813 */     fn dummy_ext(&self, macro_kind: MacroKind) -> Arc<SyntaxExtension> {
/* FP:lib.rs-1814 */         match macro_kind {
/* FP:lib.rs-1815 */             MacroKind::Bang => Arc::clone(&self.dummy_ext_bang),
/* FP:lib.rs-1816 */             MacroKind::Derive => Arc::clone(&self.dummy_ext_derive),
/* FP:lib.rs-1817 */             MacroKind::Attr => Arc::clone(&self.non_macro_attr.ext),
/* FP:lib.rs-1818 */         }
/* FP:lib.rs-1819 */     }
/* FP:lib.rs-1820 */ 
/* FP:lib.rs-1821 */     /// Returns a conditionally mutable resolver.
/* FP:lib.rs-1822 */     ///
/* FP:lib.rs-1823 */     /// Currently only dependent on `assert_speculative`, if `assert_speculative` is false,
/* FP:lib.rs-1824 */     /// the resolver will allow mutation; otherwise, it will be immutable.
/* FP:lib.rs-1825 */     fn cm(&mut self) -> CmResolver<'_, 'ra, 'tcx> {
/* FP:lib.rs-1826 */         CmResolver::new(self, !self.assert_speculative)
/* FP:lib.rs-1827 */     }
/* FP:lib.rs-1828 */ 
/* FP:lib.rs-1829 */     /// Runs the function on each namespace.
/* FP:lib.rs-1830 */     fn per_ns<F: FnMut(&mut Self, Namespace)>(&mut self, mut f: F) {
/* FP:lib.rs-1831 */         f(self, TypeNS);
/* FP:lib.rs-1832 */         f(self, ValueNS);
/* FP:lib.rs-1833 */         f(self, MacroNS);
/* FP:lib.rs-1834 */     }
/* FP:lib.rs-1835 */ 
/* FP:lib.rs-1836 */     fn per_ns_cm<'r, F: FnMut(&mut CmResolver<'r, 'ra, 'tcx>, Namespace)>(
/* FP:lib.rs-1837 */         mut self: CmResolver<'r, 'ra, 'tcx>,
/* FP:lib.rs-1838 */         mut f: F,
/* FP:lib.rs-1839 */     ) {
/* FP:lib.rs-1840 */         f(&mut self, TypeNS);
/* FP:lib.rs-1841 */         f(&mut self, ValueNS);
/* FP:lib.rs-1842 */         f(&mut self, MacroNS);
/* FP:lib.rs-1843 */     }
/* FP:lib.rs-1844 */ 
/* FP:lib.rs-1845 */     fn is_builtin_macro(&self, res: Res) -> bool {
/* FP:lib.rs-1846 */         self.get_macro(res).is_some_and(|macro_data| macro_data.ext.builtin_name.is_some())
/* FP:lib.rs-1847 */     }
/* FP:lib.rs-1848 */ 
/* FP:lib.rs-1849 */     fn macro_def(&self, mut ctxt: SyntaxContext) -> DefId {
/* FP:lib.rs-1850 */         loop {
/* FP:lib.rs-1851 */             match ctxt.outer_expn_data().macro_def_id {
/* FP:lib.rs-1852 */                 Some(def_id) => return def_id,
/* FP:lib.rs-1853 */                 None => ctxt.remove_mark(),
/* FP:lib.rs-1854 */             };
/* FP:lib.rs-1855 */         }
/* FP:lib.rs-1856 */     }
/* FP:lib.rs-1857 */ 
/* FP:lib.rs-1858 */     /// Entry point to crate resolution.
/* FP:lib.rs-1859 */     pub fn resolve_crate(&mut self, krate: &Crate) {
/* FP:lib.rs-1860 */         self.tcx.sess.time("resolve_crate", || {
/* FP:lib.rs-1861 */             self.tcx.sess.time("finalize_imports", || self.finalize_imports());
/* FP:lib.rs-1862 */             let exported_ambiguities = self.tcx.sess.time("compute_effective_visibilities", || {
/* FP:lib.rs-1863 */                 EffectiveVisibilitiesVisitor::compute_effective_visibilities(self, krate)
/* FP:lib.rs-1864 */             });
/* FP:lib.rs-1865 */             self.tcx.sess.time("lint_reexports", || self.lint_reexports(exported_ambiguities));
/* FP:lib.rs-1866 */             self.tcx
/* FP:lib.rs-1867 */                 .sess
/* FP:lib.rs-1868 */                 .time("finalize_macro_resolutions", || self.finalize_macro_resolutions(krate));
/* FP:lib.rs-1869 */             self.tcx.sess.time("late_resolve_crate", || self.late_resolve_crate(krate));
/* FP:lib.rs-1870 */             self.tcx.sess.time("resolve_main", || self.resolve_main());
/* FP:lib.rs-1871 */             self.tcx.sess.time("resolve_check_unused", || self.check_unused(krate));
/* FP:lib.rs-1872 */             self.tcx.sess.time("resolve_report_errors", || self.report_errors(krate));
/* FP:lib.rs-1873 */             self.tcx
/* FP:lib.rs-1874 */                 .sess
/* FP:lib.rs-1875 */                 .time("resolve_postprocess", || self.cstore_mut().postprocess(self.tcx, krate));
/* FP:lib.rs-1876 */         });
/* FP:lib.rs-1877 */ 
/* FP:lib.rs-1878 */         // Make sure we don't mutate the cstore from here on.
/* FP:lib.rs-1879 */         self.tcx.untracked().cstore.freeze();
/* FP:lib.rs-1880 */     }
/* FP:lib.rs-1881 */ 
/* FP:lib.rs-1882 */     fn traits_in_scope(
/* FP:lib.rs-1883 */         &mut self,
/* FP:lib.rs-1884 */         current_trait: Option<Module<'ra>>,
/* FP:lib.rs-1885 */         parent_scope: &ParentScope<'ra>,
/* FP:lib.rs-1886 */         ctxt: SyntaxContext,
/* FP:lib.rs-1887 */         assoc_item: Option<(Symbol, Namespace)>,
/* FP:lib.rs-1888 */     ) -> Vec<TraitCandidate> {
/* FP:lib.rs-1889 */         let mut found_traits = Vec::new();
/* FP:lib.rs-1890 */ 
/* FP:lib.rs-1891 */         if let Some(module) = current_trait {
/* FP:lib.rs-1892 */             if self.trait_may_have_item(Some(module), assoc_item) {
/* FP:lib.rs-1893 */                 let def_id = module.def_id();
/* FP:lib.rs-1894 */                 found_traits.push(TraitCandidate { def_id, import_ids: smallvec![] });
/* FP:lib.rs-1895 */             }
/* FP:lib.rs-1896 */         }
/* FP:lib.rs-1897 */ 
/* FP:lib.rs-1898 */         let scope_set = ScopeSet::All(TypeNS);
/* FP:lib.rs-1899 */         self.cm().visit_scopes(scope_set, parent_scope, ctxt, None, |this, scope, _, _| {
/* FP:lib.rs-1900 */             match scope {
/* FP:lib.rs-1901 */                 Scope::Module(module, _) => {
/* FP:lib.rs-1902 */                     this.get_mut().traits_in_module(module, assoc_item, &mut found_traits);
/* FP:lib.rs-1903 */                 }
/* FP:lib.rs-1904 */                 Scope::StdLibPrelude => {
/* FP:lib.rs-1905 */                     if let Some(module) = this.prelude {
/* FP:lib.rs-1906 */                         this.get_mut().traits_in_module(module, assoc_item, &mut found_traits);
/* FP:lib.rs-1907 */                     }
/* FP:lib.rs-1908 */                 }
/* FP:lib.rs-1909 */                 Scope::ExternPreludeItems
/* FP:lib.rs-1910 */                 | Scope::ExternPreludeFlags
/* FP:lib.rs-1911 */                 | Scope::ToolPrelude
/* FP:lib.rs-1912 */                 | Scope::BuiltinTypes => {}
/* FP:lib.rs-1913 */                 _ => unreachable!(),
/* FP:lib.rs-1914 */             }
/* FP:lib.rs-1915 */             None::<()>
/* FP:lib.rs-1916 */         });
/* FP:lib.rs-1917 */ 
/* FP:lib.rs-1918 */         found_traits
/* FP:lib.rs-1919 */     }
/* FP:lib.rs-1920 */ 
/* FP:lib.rs-1921 */     fn traits_in_module(
/* FP:lib.rs-1922 */         &mut self,
/* FP:lib.rs-1923 */         module: Module<'ra>,
/* FP:lib.rs-1924 */         assoc_item: Option<(Symbol, Namespace)>,
/* FP:lib.rs-1925 */         found_traits: &mut Vec<TraitCandidate>,
/* FP:lib.rs-1926 */     ) {
/* FP:lib.rs-1927 */         module.ensure_traits(self);
/* FP:lib.rs-1928 */         let traits = module.traits.borrow();
/* FP:lib.rs-1929 */         for &(trait_name, trait_binding, trait_module) in traits.as_ref().unwrap().iter() {
/* FP:lib.rs-1930 */             if self.trait_may_have_item(trait_module, assoc_item) {
/* FP:lib.rs-1931 */                 let def_id = trait_binding.res().def_id();
/* FP:lib.rs-1932 */                 let import_ids = self.find_transitive_imports(&trait_binding.kind, trait_name.0);
/* FP:lib.rs-1933 */                 found_traits.push(TraitCandidate { def_id, import_ids });
/* FP:lib.rs-1934 */             }
/* FP:lib.rs-1935 */         }
/* FP:lib.rs-1936 */     }
/* FP:lib.rs-1937 */ 
/* FP:lib.rs-1938 */     // List of traits in scope is pruned on best effort basis. We reject traits not having an
/* FP:lib.rs-1939 */     // associated item with the given name and namespace (if specified). This is a conservative
/* FP:lib.rs-1940 */     // optimization, proper hygienic type-based resolution of associated items is done in typeck.
/* FP:lib.rs-1941 */     // We don't reject trait aliases (`trait_module == None`) because we don't have access to their
/* FP:lib.rs-1942 */     // associated items.
/* FP:lib.rs-1943 */     fn trait_may_have_item(
/* FP:lib.rs-1944 */         &self,
/* FP:lib.rs-1945 */         trait_module: Option<Module<'ra>>,
/* FP:lib.rs-1946 */         assoc_item: Option<(Symbol, Namespace)>,
/* FP:lib.rs-1947 */     ) -> bool {
/* FP:lib.rs-1948 */         match (trait_module, assoc_item) {
/* FP:lib.rs-1949 */             (Some(trait_module), Some((name, ns))) => self
/* FP:lib.rs-1950 */                 .resolutions(trait_module)
/* FP:lib.rs-1951 */                 .borrow()
/* FP:lib.rs-1952 */                 .iter()
/* FP:lib.rs-1953 */                 .any(|(key, _name_resolution)| key.ns == ns && key.ident.name == name),
/* FP:lib.rs-1954 */             _ => true,
/* FP:lib.rs-1955 */         }
/* FP:lib.rs-1956 */     }
/* FP:lib.rs-1957 */ 
/* FP:lib.rs-1958 */     fn find_transitive_imports(
/* FP:lib.rs-1959 */         &mut self,
/* FP:lib.rs-1960 */         mut kind: &NameBindingKind<'_>,
/* FP:lib.rs-1961 */         trait_name: Ident,
/* FP:lib.rs-1962 */     ) -> SmallVec<[LocalDefId; 1]> {
/* FP:lib.rs-1963 */         let mut import_ids = smallvec![];
/* FP:lib.rs-1964 */         while let NameBindingKind::Import { import, binding, .. } = kind {
/* FP:lib.rs-1965 */             if let Some(node_id) = import.id() {
/* FP:lib.rs-1966 */                 let def_id = self.local_def_id(node_id);
/* FP:lib.rs-1967 */                 self.maybe_unused_trait_imports.insert(def_id);
/* FP:lib.rs-1968 */                 import_ids.push(def_id);
/* FP:lib.rs-1969 */             }
/* FP:lib.rs-1970 */             self.add_to_glob_map(*import, trait_name);
/* FP:lib.rs-1971 */             kind = &binding.kind;
/* FP:lib.rs-1972 */         }
/* FP:lib.rs-1973 */         import_ids
/* FP:lib.rs-1974 */     }
/* FP:lib.rs-1975 */ 
/* FP:lib.rs-1976 */     fn resolutions(&self, module: Module<'ra>) -> &'ra Resolutions<'ra> {
/* FP:lib.rs-1977 */         if module.populate_on_access.get() {
/* FP:lib.rs-1978 */             module.populate_on_access.set(false);
/* FP:lib.rs-1979 */             self.build_reduced_graph_external(module);
/* FP:lib.rs-1980 */         }
/* FP:lib.rs-1981 */         &module.0.0.lazy_resolutions
/* FP:lib.rs-1982 */     }
/* FP:lib.rs-1983 */ 
/* FP:lib.rs-1984 */     fn resolution(
/* FP:lib.rs-1985 */         &self,
/* FP:lib.rs-1986 */         module: Module<'ra>,
/* FP:lib.rs-1987 */         key: BindingKey,
/* FP:lib.rs-1988 */     ) -> Option<Ref<'ra, NameResolution<'ra>>> {
/* FP:lib.rs-1989 */         self.resolutions(module).borrow().get(&key).map(|resolution| resolution.borrow())
/* FP:lib.rs-1990 */     }
/* FP:lib.rs-1991 */ 
/* FP:lib.rs-1992 */     fn resolution_or_default(
/* FP:lib.rs-1993 */         &self,
/* FP:lib.rs-1994 */         module: Module<'ra>,
/* FP:lib.rs-1995 */         key: BindingKey,
/* FP:lib.rs-1996 */     ) -> &'ra RefCell<NameResolution<'ra>> {
/* FP:lib.rs-1997 */         self.resolutions(module)
/* FP:lib.rs-1998 */             .borrow_mut()
/* FP:lib.rs-1999 */             .entry(key)
/* FP:lib.rs-2000 */             .or_insert_with(|| self.arenas.alloc_name_resolution())
/* FP:lib.rs-2001 */     }
/* FP:lib.rs-2002 */ 
/* FP:lib.rs-2003 */     /// Test if AmbiguityError ambi is any identical to any one inside ambiguity_errors
/* FP:lib.rs-2004 */     fn matches_previous_ambiguity_error(&self, ambi: &AmbiguityError<'_>) -> bool {
/* FP:lib.rs-2005 */         for ambiguity_error in &self.ambiguity_errors {
/* FP:lib.rs-2006 */             // if the span location and ident as well as its span are the same
/* FP:lib.rs-2007 */             if ambiguity_error.kind == ambi.kind
/* FP:lib.rs-2008 */                 && ambiguity_error.ident == ambi.ident
/* FP:lib.rs-2009 */                 && ambiguity_error.ident.span == ambi.ident.span
/* FP:lib.rs-2010 */                 && ambiguity_error.b1.span == ambi.b1.span
/* FP:lib.rs-2011 */                 && ambiguity_error.b2.span == ambi.b2.span
/* FP:lib.rs-2012 */                 && ambiguity_error.misc1 == ambi.misc1
/* FP:lib.rs-2013 */                 && ambiguity_error.misc2 == ambi.misc2
/* FP:lib.rs-2014 */             {
/* FP:lib.rs-2015 */                 return true;
/* FP:lib.rs-2016 */             }
/* FP:lib.rs-2017 */         }
/* FP:lib.rs-2018 */         false
/* FP:lib.rs-2019 */     }
/* FP:lib.rs-2020 */ 
/* FP:lib.rs-2021 */     fn record_use(&mut self, ident: Ident, used_binding: NameBinding<'ra>, used: Used) {
/* FP:lib.rs-2022 */         self.record_use_inner(ident, used_binding, used, used_binding.warn_ambiguity);
/* FP:lib.rs-2023 */     }
/* FP:lib.rs-2024 */ 
/* FP:lib.rs-2025 */     fn record_use_inner(
/* FP:lib.rs-2026 */         &mut self,
/* FP:lib.rs-2027 */         ident: Ident,
/* FP:lib.rs-2028 */         used_binding: NameBinding<'ra>,
/* FP:lib.rs-2029 */         used: Used,
/* FP:lib.rs-2030 */         warn_ambiguity: bool,
/* FP:lib.rs-2031 */     ) {
/* FP:lib.rs-2032 */         if let Some((b2, kind)) = used_binding.ambiguity {
/* FP:lib.rs-2033 */             let ambiguity_error = AmbiguityError {
/* FP:lib.rs-2034 */                 kind,
/* FP:lib.rs-2035 */                 ident,
/* FP:lib.rs-2036 */                 b1: used_binding,
/* FP:lib.rs-2037 */                 b2,
/* FP:lib.rs-2038 */                 misc1: AmbiguityErrorMisc::None,
/* FP:lib.rs-2039 */                 misc2: AmbiguityErrorMisc::None,
/* FP:lib.rs-2040 */                 warning: warn_ambiguity,
/* FP:lib.rs-2041 */             };
/* FP:lib.rs-2042 */             if !self.matches_previous_ambiguity_error(&ambiguity_error) {
/* FP:lib.rs-2043 */                 // avoid duplicated span information to be emit out
/* FP:lib.rs-2044 */                 self.ambiguity_errors.push(ambiguity_error);
/* FP:lib.rs-2045 */             }
/* FP:lib.rs-2046 */         }
/* FP:lib.rs-2047 */         if let NameBindingKind::Import { import, binding } = used_binding.kind {
/* FP:lib.rs-2048 */             if let ImportKind::MacroUse { warn_private: true } = import.kind {
/* FP:lib.rs-2049 */                 // Do not report the lint if the macro name resolves in stdlib prelude
/* FP:lib.rs-2050 */                 // even without the problematic `macro_use` import.
/* FP:lib.rs-2051 */                 let found_in_stdlib_prelude = self.prelude.is_some_and(|prelude| {
/* FP:lib.rs-2052 */                     let empty_module = self.empty_module;
/* FP:lib.rs-2053 */                     let arenas = self.arenas;
/* FP:lib.rs-2054 */                     self.cm()
/* FP:lib.rs-2055 */                         .maybe_resolve_ident_in_module(
/* FP:lib.rs-2056 */                             ModuleOrUniformRoot::Module(prelude),
/* FP:lib.rs-2057 */                             ident,
/* FP:lib.rs-2058 */                             MacroNS,
/* FP:lib.rs-2059 */                             &ParentScope::module(empty_module, arenas),
/* FP:lib.rs-2060 */                             None,
/* FP:lib.rs-2061 */                         )
/* FP:lib.rs-2062 */                         .is_ok()
/* FP:lib.rs-2063 */                 });
/* FP:lib.rs-2064 */                 if !found_in_stdlib_prelude {
/* FP:lib.rs-2065 */                     self.lint_buffer().buffer_lint(
/* FP:lib.rs-2066 */                         PRIVATE_MACRO_USE,
/* FP:lib.rs-2067 */                         import.root_id,
/* FP:lib.rs-2068 */                         ident.span,
/* FP:lib.rs-2069 */                         BuiltinLintDiag::MacroIsPrivate(ident),
/* FP:lib.rs-2070 */                     );
/* FP:lib.rs-2071 */                 }
/* FP:lib.rs-2072 */             }
/* FP:lib.rs-2073 */             // Avoid marking `extern crate` items that refer to a name from extern prelude,
/* FP:lib.rs-2074 */             // but not introduce it, as used if they are accessed from lexical scope.
/* FP:lib.rs-2075 */             if used == Used::Scope
/* FP:lib.rs-2076 */                 && let Some(entry) = self.extern_prelude.get(&Macros20NormalizedIdent::new(ident))
/* FP:lib.rs-2077 */                 && entry.item_binding == Some((used_binding, false))
/* FP:lib.rs-2078 */             {
/* FP:lib.rs-2079 */                 return;
/* FP:lib.rs-2080 */             }
/* FP:lib.rs-2081 */             let old_used = self.import_use_map.entry(import).or_insert(used);
/* FP:lib.rs-2082 */             if *old_used < used {
/* FP:lib.rs-2083 */                 *old_used = used;
/* FP:lib.rs-2084 */             }
/* FP:lib.rs-2085 */             if let Some(id) = import.id() {
/* FP:lib.rs-2086 */                 self.used_imports.insert(id);
/* FP:lib.rs-2087 */             }
/* FP:lib.rs-2088 */             self.add_to_glob_map(import, ident);
/* FP:lib.rs-2089 */             self.record_use_inner(
/* FP:lib.rs-2090 */                 ident,
/* FP:lib.rs-2091 */                 binding,
/* FP:lib.rs-2092 */                 Used::Other,
/* FP:lib.rs-2093 */                 warn_ambiguity || binding.warn_ambiguity,
/* FP:lib.rs-2094 */             );
/* FP:lib.rs-2095 */         }
/* FP:lib.rs-2096 */     }
/* FP:lib.rs-2097 */ 
/* FP:lib.rs-2098 */     #[inline]
/* FP:lib.rs-2099 */     fn add_to_glob_map(&mut self, import: Import<'_>, ident: Ident) {
/* FP:lib.rs-2100 */         if let ImportKind::Glob { id, .. } = import.kind {
/* FP:lib.rs-2101 */             let def_id = self.local_def_id(id);
/* FP:lib.rs-2102 */             self.glob_map.entry(def_id).or_default().insert(ident.name);
/* FP:lib.rs-2103 */         }
/* FP:lib.rs-2104 */     }
/* FP:lib.rs-2105 */ 
/* FP:lib.rs-2106 */     fn resolve_crate_root(&self, ident: Ident) -> Module<'ra> {
/* FP:lib.rs-2107 */         debug!("resolve_crate_root({:?})", ident);
/* FP:lib.rs-2108 */         let mut ctxt = ident.span.ctxt();
/* FP:lib.rs-2109 */         let mark = if ident.name == kw::DollarCrate {
/* FP:lib.rs-2110 */             // When resolving `$crate` from a `macro_rules!` invoked in a `macro`,
/* FP:lib.rs-2111 */             // we don't want to pretend that the `macro_rules!` definition is in the `macro`
/* FP:lib.rs-2112 */             // as described in `SyntaxContext::apply_mark`, so we ignore prepended opaque marks.
/* FP:lib.rs-2113 */             // FIXME: This is only a guess and it doesn't work correctly for `macro_rules!`
/* FP:lib.rs-2114 */             // definitions actually produced by `macro` and `macro` definitions produced by
/* FP:lib.rs-2115 */             // `macro_rules!`, but at least such configurations are not stable yet.
/* FP:lib.rs-2116 */             ctxt = ctxt.normalize_to_macro_rules();
/* FP:lib.rs-2117 */             debug!(
/* FP:lib.rs-2118 */                 "resolve_crate_root: marks={:?}",
/* FP:lib.rs-2119 */                 ctxt.marks().into_iter().map(|(i, t)| (i.expn_data(), t)).collect::<Vec<_>>()
/* FP:lib.rs-2120 */             );
/* FP:lib.rs-2121 */             let mut iter = ctxt.marks().into_iter().rev().peekable();
/* FP:lib.rs-2122 */             let mut result = None;
/* FP:lib.rs-2123 */             // Find the last opaque mark from the end if it exists.
/* FP:lib.rs-2124 */             while let Some(&(mark, transparency)) = iter.peek() {
/* FP:lib.rs-2125 */                 if transparency == Transparency::Opaque {
/* FP:lib.rs-2126 */                     result = Some(mark);
/* FP:lib.rs-2127 */                     iter.next();
/* FP:lib.rs-2128 */                 } else {
/* FP:lib.rs-2129 */                     break;
/* FP:lib.rs-2130 */                 }
/* FP:lib.rs-2131 */             }
/* FP:lib.rs-2132 */             debug!(
/* FP:lib.rs-2133 */                 "resolve_crate_root: found opaque mark {:?} {:?}",
/* FP:lib.rs-2134 */                 result,
/* FP:lib.rs-2135 */                 result.map(|r| r.expn_data())
/* FP:lib.rs-2136 */             );
/* FP:lib.rs-2137 */             // Then find the last semi-opaque mark from the end if it exists.
/* FP:lib.rs-2138 */             for (mark, transparency) in iter {
/* FP:lib.rs-2139 */                 if transparency == Transparency::SemiOpaque {
/* FP:lib.rs-2140 */                     result = Some(mark);
/* FP:lib.rs-2141 */                 } else {
/* FP:lib.rs-2142 */                     break;
/* FP:lib.rs-2143 */                 }
/* FP:lib.rs-2144 */             }
/* FP:lib.rs-2145 */             debug!(
/* FP:lib.rs-2146 */                 "resolve_crate_root: found semi-opaque mark {:?} {:?}",
/* FP:lib.rs-2147 */                 result,
/* FP:lib.rs-2148 */                 result.map(|r| r.expn_data())
/* FP:lib.rs-2149 */             );
/* FP:lib.rs-2150 */             result
/* FP:lib.rs-2151 */         } else {
/* FP:lib.rs-2152 */             debug!("resolve_crate_root: not DollarCrate");
/* FP:lib.rs-2153 */             ctxt = ctxt.normalize_to_macros_2_0();
/* FP:lib.rs-2154 */             ctxt.adjust(ExpnId::root())
/* FP:lib.rs-2155 */         };
/* FP:lib.rs-2156 */         let module = match mark {
/* FP:lib.rs-2157 */             Some(def) => self.expn_def_scope(def),
/* FP:lib.rs-2158 */             None => {
/* FP:lib.rs-2159 */                 debug!(
/* FP:lib.rs-2160 */                     "resolve_crate_root({:?}): found no mark (ident.span = {:?})",
/* FP:lib.rs-2161 */                     ident, ident.span
/* FP:lib.rs-2162 */                 );
/* FP:lib.rs-2163 */                 return self.graph_root;
/* FP:lib.rs-2164 */             }
/* FP:lib.rs-2165 */         };
/* FP:lib.rs-2166 */         let module = self.expect_module(
/* FP:lib.rs-2167 */             module.opt_def_id().map_or(LOCAL_CRATE, |def_id| def_id.krate).as_def_id(),
/* FP:lib.rs-2168 */         );
/* FP:lib.rs-2169 */         debug!(
/* FP:lib.rs-2170 */             "resolve_crate_root({:?}): got module {:?} ({:?}) (ident.span = {:?})",
/* FP:lib.rs-2171 */             ident,
/* FP:lib.rs-2172 */             module,
/* FP:lib.rs-2173 */             module.kind.name(),
/* FP:lib.rs-2174 */             ident.span
/* FP:lib.rs-2175 */         );
/* FP:lib.rs-2176 */         module
/* FP:lib.rs-2177 */     }
/* FP:lib.rs-2178 */ 
/* FP:lib.rs-2179 */     fn resolve_self(&self, ctxt: &mut SyntaxContext, module: Module<'ra>) -> Module<'ra> {
/* FP:lib.rs-2180 */         let mut module = self.expect_module(module.nearest_parent_mod());
/* FP:lib.rs-2181 */         while module.span.ctxt().normalize_to_macros_2_0() != *ctxt {
/* FP:lib.rs-2182 */             let parent = module.parent.unwrap_or_else(|| self.expn_def_scope(ctxt.remove_mark()));
/* FP:lib.rs-2183 */             module = self.expect_module(parent.nearest_parent_mod());
/* FP:lib.rs-2184 */         }
/* FP:lib.rs-2185 */         module
/* FP:lib.rs-2186 */     }
/* FP:lib.rs-2187 */ 
/* FP:lib.rs-2188 */     fn record_partial_res(&mut self, node_id: NodeId, resolution: PartialRes) {
/* FP:lib.rs-2189 */         debug!("(recording res) recording {:?} for {}", resolution, node_id);
/* FP:lib.rs-2190 */         if let Some(prev_res) = self.partial_res_map.insert(node_id, resolution) {
/* FP:lib.rs-2191 */             panic!("path resolved multiple times ({prev_res:?} before, {resolution:?} now)");
/* FP:lib.rs-2192 */         }
/* FP:lib.rs-2193 */     }
/* FP:lib.rs-2194 */ 
/* FP:lib.rs-2195 */     fn record_pat_span(&mut self, node: NodeId, span: Span) {
/* FP:lib.rs-2196 */         debug!("(recording pat) recording {:?} for {:?}", node, span);
/* FP:lib.rs-2197 */         self.pat_span_map.insert(node, span);
/* FP:lib.rs-2198 */     }
/* FP:lib.rs-2199 */ 
/* FP:lib.rs-2200 */     fn is_accessible_from(&self, vis: Visibility<impl Into<DefId>>, module: Module<'ra>) -> bool {
/* FP:lib.rs-2201 */         vis.is_accessible_from(module.nearest_parent_mod(), self.tcx)
/* FP:lib.rs-2202 */     }
/* FP:lib.rs-2203 */ 
/* FP:lib.rs-2204 */     fn set_binding_parent_module(&mut self, binding: NameBinding<'ra>, module: Module<'ra>) {
/* FP:lib.rs-2205 */         if let Some(old_module) = self.binding_parent_modules.insert(binding, module) {
/* FP:lib.rs-2206 */             if module != old_module {
/* FP:lib.rs-2207 */                 span_bug!(binding.span, "parent module is reset for binding");
/* FP:lib.rs-2208 */             }
/* FP:lib.rs-2209 */         }
/* FP:lib.rs-2210 */     }
/* FP:lib.rs-2211 */ 
/* FP:lib.rs-2212 */     fn disambiguate_macro_rules_vs_modularized(
/* FP:lib.rs-2213 */         &self,
/* FP:lib.rs-2214 */         macro_rules: NameBinding<'ra>,
/* FP:lib.rs-2215 */         modularized: NameBinding<'ra>,
/* FP:lib.rs-2216 */     ) -> bool {
/* FP:lib.rs-2217 */         // Some non-controversial subset of ambiguities "modularized macro name" vs "macro_rules"
/* FP:lib.rs-2218 */         // is disambiguated to mitigate regressions from macro modularization.
/* FP:lib.rs-2219 */         // Scoping for `macro_rules` behaves like scoping for `let` at module level, in general.
/* FP:lib.rs-2220 */         match (
/* FP:lib.rs-2221 */             self.binding_parent_modules.get(&macro_rules),
/* FP:lib.rs-2222 */             self.binding_parent_modules.get(&modularized),
/* FP:lib.rs-2223 */         ) {
/* FP:lib.rs-2224 */             (Some(macro_rules), Some(modularized)) => {
/* FP:lib.rs-2225 */                 macro_rules.nearest_parent_mod() == modularized.nearest_parent_mod()
/* FP:lib.rs-2226 */                     && modularized.is_ancestor_of(*macro_rules)
/* FP:lib.rs-2227 */             }
/* FP:lib.rs-2228 */             _ => false,
/* FP:lib.rs-2229 */         }
/* FP:lib.rs-2230 */     }
/* FP:lib.rs-2231 */ 
/* FP:lib.rs-2232 */     fn extern_prelude_get_item<'r>(
/* FP:lib.rs-2233 */         mut self: CmResolver<'r, 'ra, 'tcx>,
/* FP:lib.rs-2234 */         ident: Ident,
/* FP:lib.rs-2235 */         finalize: bool,
/* FP:lib.rs-2236 */     ) -> Option<NameBinding<'ra>> {
/* FP:lib.rs-2237 */         let entry = self.extern_prelude.get(&Macros20NormalizedIdent::new(ident));
/* FP:lib.rs-2238 */         entry.and_then(|entry| entry.item_binding).map(|(binding, _)| {
/* FP:lib.rs-2239 */             if finalize {
/* FP:lib.rs-2240 */                 self.get_mut().record_use(ident, binding, Used::Scope);
/* FP:lib.rs-2241 */             }
/* FP:lib.rs-2242 */             binding
/* FP:lib.rs-2243 */         })
/* FP:lib.rs-2244 */     }
/* FP:lib.rs-2245 */ 
/* FP:lib.rs-2246 */     fn extern_prelude_get_flag(&self, ident: Ident, finalize: bool) -> Option<NameBinding<'ra>> {
/* FP:lib.rs-2247 */         let entry = self.extern_prelude.get(&Macros20NormalizedIdent::new(ident));
/* FP:lib.rs-2248 */         entry.and_then(|entry| entry.flag_binding.as_ref()).and_then(|flag_binding| {
/* FP:lib.rs-2249 */             let (pending_binding, finalized) = flag_binding.get();
/* FP:lib.rs-2250 */             let binding = match pending_binding {
/* FP:lib.rs-2251 */                 PendingBinding::Ready(binding) => {
/* FP:lib.rs-2252 */                     if finalize && !finalized {
/* FP:lib.rs-2253 */                         self.cstore_mut().process_path_extern(self.tcx, ident.name, ident.span);
/* FP:lib.rs-2254 */                     }
/* FP:lib.rs-2255 */                     binding
/* FP:lib.rs-2256 */                 }
/* FP:lib.rs-2257 */                 PendingBinding::Pending => {
/* FP:lib.rs-2258 */                     debug_assert!(!finalized);
/* FP:lib.rs-2259 */                     let crate_id = if finalize {
/* FP:lib.rs-2260 */                         self.cstore_mut().process_path_extern(self.tcx, ident.name, ident.span)
/* FP:lib.rs-2261 */                     } else {
/* FP:lib.rs-2262 */                         self.cstore_mut().maybe_process_path_extern(self.tcx, ident.name)
/* FP:lib.rs-2263 */                     };
/* FP:lib.rs-2264 */                     crate_id.map(|crate_id| {
/* FP:lib.rs-2265 */                         let res = Res::Def(DefKind::Mod, crate_id.as_def_id());
/* FP:lib.rs-2266 */                         self.arenas.new_pub_res_binding(res, DUMMY_SP, LocalExpnId::ROOT)
/* FP:lib.rs-2267 */                     })
/* FP:lib.rs-2268 */                 }
/* FP:lib.rs-2269 */             };
/* FP:lib.rs-2270 */             flag_binding.set((PendingBinding::Ready(binding), finalize || finalized));
/* FP:lib.rs-2271 */             binding.or_else(|| finalize.then_some(self.dummy_binding))
/* FP:lib.rs-2272 */         })
/* FP:lib.rs-2273 */     }
/* FP:lib.rs-2274 */ 
/* FP:lib.rs-2275 */     /// Rustdoc uses this to resolve doc link paths in a recoverable way. `PathResult<'a>`
/* FP:lib.rs-2276 */     /// isn't something that can be returned because it can't be made to live that long,
/* FP:lib.rs-2277 */     /// and also it's a private type. Fortunately rustdoc doesn't need to know the error,
/* FP:lib.rs-2278 */     /// just that an error occurred.
/* FP:lib.rs-2279 */     fn resolve_rustdoc_path(
/* FP:lib.rs-2280 */         &mut self,
/* FP:lib.rs-2281 */         path_str: &str,
/* FP:lib.rs-2282 */         ns: Namespace,
/* FP:lib.rs-2283 */         parent_scope: ParentScope<'ra>,
/* FP:lib.rs-2284 */     ) -> Option<Res> {
/* FP:lib.rs-2285 */         let segments: Result<Vec<_>, ()> = path_str
/* FP:lib.rs-2286 */             .split("::")
/* FP:lib.rs-2287 */             .enumerate()
/* FP:lib.rs-2288 */             .map(|(i, s)| {
/* FP:lib.rs-2289 */                 let sym = if s.is_empty() {
/* FP:lib.rs-2290 */                     if i == 0 {
/* FP:lib.rs-2291 */                         // For a path like `::a::b`, use `kw::PathRoot` as the leading segment.
/* FP:lib.rs-2292 */                         kw::PathRoot
/* FP:lib.rs-2293 */                     } else {
/* FP:lib.rs-2294 */                         return Err(()); // occurs in cases like `String::`
/* FP:lib.rs-2295 */                     }
/* FP:lib.rs-2296 */                 } else {
/* FP:lib.rs-2297 */                     Symbol::intern(s)
/* FP:lib.rs-2298 */                 };
/* FP:lib.rs-2299 */                 Ok(Segment::from_ident(Ident::with_dummy_span(sym)))
/* FP:lib.rs-2300 */             })
/* FP:lib.rs-2301 */             .collect();
/* FP:lib.rs-2302 */         let Ok(segments) = segments else { return None };
/* FP:lib.rs-2303 */ 
/* FP:lib.rs-2304 */         match self.cm().maybe_resolve_path(&segments, Some(ns), &parent_scope, None) {
/* FP:lib.rs-2305 */             PathResult::Module(ModuleOrUniformRoot::Module(module)) => Some(module.res().unwrap()),
/* FP:lib.rs-2306 */             PathResult::NonModule(path_res) => {
/* FP:lib.rs-2307 */                 path_res.full_res().filter(|res| !matches!(res, Res::Def(DefKind::Ctor(..), _)))
/* FP:lib.rs-2308 */             }
/* FP:lib.rs-2309 */             PathResult::Module(ModuleOrUniformRoot::ExternPrelude) | PathResult::Failed { .. } => {
/* FP:lib.rs-2310 */                 None
/* FP:lib.rs-2311 */             }
/* FP:lib.rs-2312 */             PathResult::Module(..) | PathResult::Indeterminate => unreachable!(),
/* FP:lib.rs-2313 */         }
/* FP:lib.rs-2314 */     }
/* FP:lib.rs-2315 */ 
/* FP:lib.rs-2316 */     /// Retrieves definition span of the given `DefId`.
/* FP:lib.rs-2317 */     fn def_span(&self, def_id: DefId) -> Span {
/* FP:lib.rs-2318 */         match def_id.as_local() {
/* FP:lib.rs-2319 */             Some(def_id) => self.tcx.source_span(def_id),
/* FP:lib.rs-2320 */             // Query `def_span` is not used because hashing its result span is expensive.
/* FP:lib.rs-2321 */             None => self.cstore().def_span_untracked(def_id, self.tcx.sess),
/* FP:lib.rs-2322 */         }
/* FP:lib.rs-2323 */     }
/* FP:lib.rs-2324 */ 
/* FP:lib.rs-2325 */     fn field_idents(&self, def_id: DefId) -> Option<Vec<Ident>> {
/* FP:lib.rs-2326 */         match def_id.as_local() {
/* FP:lib.rs-2327 */             Some(def_id) => self.field_names.get(&def_id).cloned(),
/* FP:lib.rs-2328 */             None => Some(
/* FP:lib.rs-2329 */                 self.tcx
/* FP:lib.rs-2330 */                     .associated_item_def_ids(def_id)
/* FP:lib.rs-2331 */                     .iter()
/* FP:lib.rs-2332 */                     .map(|&def_id| {
/* FP:lib.rs-2333 */                         Ident::new(self.tcx.item_name(def_id), self.tcx.def_span(def_id))
/* FP:lib.rs-2334 */                     })
/* FP:lib.rs-2335 */                     .collect(),
/* FP:lib.rs-2336 */             ),
/* FP:lib.rs-2337 */         }
/* FP:lib.rs-2338 */     }
/* FP:lib.rs-2339 */ 
/* FP:lib.rs-2340 */     fn field_defaults(&self, def_id: DefId) -> Option<Vec<Symbol>> {
/* FP:lib.rs-2341 */         match def_id.as_local() {
/* FP:lib.rs-2342 */             Some(def_id) => self.field_defaults.get(&def_id).cloned(),
/* FP:lib.rs-2343 */             None => Some(
/* FP:lib.rs-2344 */                 self.tcx
/* FP:lib.rs-2345 */                     .associated_item_def_ids(def_id)
/* FP:lib.rs-2346 */                     .iter()
/* FP:lib.rs-2347 */                     .filter_map(|&def_id| {
/* FP:lib.rs-2348 */                         self.tcx.default_field(def_id).map(|_| self.tcx.item_name(def_id))
/* FP:lib.rs-2349 */                     })
/* FP:lib.rs-2350 */                     .collect(),
/* FP:lib.rs-2351 */             ),
/* FP:lib.rs-2352 */         }
/* FP:lib.rs-2353 */     }
/* FP:lib.rs-2354 */ 
/* FP:lib.rs-2355 */     /// Checks if an expression refers to a function marked with
/* FP:lib.rs-2356 */     /// `#[rustc_legacy_const_generics]` and returns the argument index list
/* FP:lib.rs-2357 */     /// from the attribute.
/* FP:lib.rs-2358 */     fn legacy_const_generic_args(&mut self, expr: &Expr) -> Option<Vec<usize>> {
/* FP:lib.rs-2359 */         if let ExprKind::Path(None, path) = &expr.kind {
/* FP:lib.rs-2360 */             // Don't perform legacy const generics rewriting if the path already
/* FP:lib.rs-2361 */             // has generic arguments.
/* FP:lib.rs-2362 */             if path.segments.last().unwrap().args.is_some() {
/* FP:lib.rs-2363 */                 return None;
/* FP:lib.rs-2364 */             }
/* FP:lib.rs-2365 */ 
/* FP:lib.rs-2366 */             let res = self.partial_res_map.get(&expr.id)?.full_res()?;
/* FP:lib.rs-2367 */             if let Res::Def(def::DefKind::Fn, def_id) = res {
/* FP:lib.rs-2368 */                 // We only support cross-crate argument rewriting. Uses
/* FP:lib.rs-2369 */                 // within the same crate should be updated to use the new
/* FP:lib.rs-2370 */                 // const generics style.
/* FP:lib.rs-2371 */                 if def_id.is_local() {
/* FP:lib.rs-2372 */                     return None;
/* FP:lib.rs-2373 */                 }
/* FP:lib.rs-2374 */ 
/* FP:lib.rs-2375 */                 if let Some(v) = self.legacy_const_generic_args.get(&def_id) {
/* FP:lib.rs-2376 */                     return v.clone();
/* FP:lib.rs-2377 */                 }
/* FP:lib.rs-2378 */ 
/* FP:lib.rs-2379 */                 let attr = self.tcx.get_attr(def_id, sym::rustc_legacy_const_generics)?;
/* FP:lib.rs-2380 */                 let mut ret = Vec::new();
/* FP:lib.rs-2381 */                 for meta in attr.meta_item_list()? {
/* FP:lib.rs-2382 */                     match meta.lit()?.kind {
/* FP:lib.rs-2383 */                         LitKind::Int(a, _) => ret.push(a.get() as usize),
/* FP:lib.rs-2384 */                         _ => panic!("invalid arg index"),
/* FP:lib.rs-2385 */                     }
/* FP:lib.rs-2386 */                 }
/* FP:lib.rs-2387 */                 // Cache the lookup to avoid parsing attributes for an item multiple times.
/* FP:lib.rs-2388 */                 self.legacy_const_generic_args.insert(def_id, Some(ret.clone()));
/* FP:lib.rs-2389 */                 return Some(ret);
/* FP:lib.rs-2390 */             }
/* FP:lib.rs-2391 */         }
/* FP:lib.rs-2392 */         None
/* FP:lib.rs-2393 */     }
/* FP:lib.rs-2394 */ 
/* FP:lib.rs-2395 */     fn resolve_main(&mut self) {
/* FP:lib.rs-2396 */         let module = self.graph_root;
/* FP:lib.rs-2397 */         let ident = Ident::with_dummy_span(sym::main);
/* FP:lib.rs-2398 */         let parent_scope = &ParentScope::module(module, self.arenas);
/* FP:lib.rs-2399 */ 
/* FP:lib.rs-2400 */         let Ok(name_binding) = self.cm().maybe_resolve_ident_in_module(
/* FP:lib.rs-2401 */             ModuleOrUniformRoot::Module(module),
/* FP:lib.rs-2402 */             ident,
/* FP:lib.rs-2403 */             ValueNS,
/* FP:lib.rs-2404 */             parent_scope,
/* FP:lib.rs-2405 */             None,
/* FP:lib.rs-2406 */         ) else {
/* FP:lib.rs-2407 */             return;
/* FP:lib.rs-2408 */         };
/* FP:lib.rs-2409 */ 
/* FP:lib.rs-2410 */         let res = name_binding.res();
/* FP:lib.rs-2411 */         let is_import = name_binding.is_import();
/* FP:lib.rs-2412 */         let span = name_binding.span;
/* FP:lib.rs-2413 */         if let Res::Def(DefKind::Fn, _) = res {
/* FP:lib.rs-2414 */             self.record_use(ident, name_binding, Used::Other);
/* FP:lib.rs-2415 */         }
/* FP:lib.rs-2416 */         self.main_def = Some(MainDefinition { res, is_import, span });
/* FP:lib.rs-2417 */     }
/* FP:lib.rs-2418 */ }
/* FP:lib.rs-2419 */ 
/* FP:lib.rs-2420 */ fn names_to_string(names: impl Iterator<Item = Symbol>) -> String {
/* FP:lib.rs-2421 */     let mut result = String::new();
/* FP:lib.rs-2422 */     for (i, name) in names.filter(|name| *name != kw::PathRoot).enumerate() {
/* FP:lib.rs-2423 */         if i > 0 {
/* FP:lib.rs-2424 */             result.push_str("::");
/* FP:lib.rs-2425 */         }
/* FP:lib.rs-2426 */         if Ident::with_dummy_span(name).is_raw_guess() {
/* FP:lib.rs-2427 */             result.push_str("r#");
/* FP:lib.rs-2428 */         }
/* FP:lib.rs-2429 */         result.push_str(name.as_str());
/* FP:lib.rs-2430 */     }
/* FP:lib.rs-2431 */     result
/* FP:lib.rs-2432 */ }
/* FP:lib.rs-2433 */ 
/* FP:lib.rs-2434 */ fn path_names_to_string(path: &Path) -> String {
/* FP:lib.rs-2435 */     names_to_string(path.segments.iter().map(|seg| seg.ident.name))
/* FP:lib.rs-2436 */ }
/* FP:lib.rs-2437 */ 
/* FP:lib.rs-2438 */ /// A somewhat inefficient routine to obtain the name of a module.
/* FP:lib.rs-2439 */ fn module_to_string(mut module: Module<'_>) -> Option<String> {
/* FP:lib.rs-2440 */     let mut names = Vec::new();
/* FP:lib.rs-2441 */     loop {
/* FP:lib.rs-2442 */         if let ModuleKind::Def(.., name) = module.kind {
/* FP:lib.rs-2443 */             if let Some(parent) = module.parent {
/* FP:lib.rs-2444 */                 // `unwrap` is safe: the presence of a parent means it's not the crate root.
/* FP:lib.rs-2445 */                 names.push(name.unwrap());
/* FP:lib.rs-2446 */                 module = parent
/* FP:lib.rs-2447 */             } else {
/* FP:lib.rs-2448 */                 break;
/* FP:lib.rs-2449 */             }
/* FP:lib.rs-2450 */         } else {
/* FP:lib.rs-2451 */             names.push(sym::opaque_module_name_placeholder);
/* FP:lib.rs-2452 */             let Some(parent) = module.parent else {
/* FP:lib.rs-2453 */                 return None;
/* FP:lib.rs-2454 */             };
/* FP:lib.rs-2455 */             module = parent;
/* FP:lib.rs-2456 */         }
/* FP:lib.rs-2457 */     }
/* FP:lib.rs-2458 */     if names.is_empty() {
/* FP:lib.rs-2459 */         return None;
/* FP:lib.rs-2460 */     }
/* FP:lib.rs-2461 */     Some(names_to_string(names.iter().rev().copied()))
/* FP:lib.rs-2462 */ }
/* FP:lib.rs-2463 */ 
/* FP:lib.rs-2464 */ #[derive(Copy, Clone, PartialEq, Debug)]
/* FP:lib.rs-2465 */ enum Stage {
/* FP:lib.rs-2466 */     /// Resolving an import or a macro.
/* FP:lib.rs-2467 */     /// Used when macro expansion is either not yet finished, or we are finalizing its results.
/* FP:lib.rs-2468 */     /// Used by default as a more restrictive variant that can produce additional errors.
/* FP:lib.rs-2469 */     Early,
/* FP:lib.rs-2470 */     /// Resolving something in late resolution when all imports are resolved
/* FP:lib.rs-2471 */     /// and all macros are expanded.
/* FP:lib.rs-2472 */     Late,
/* FP:lib.rs-2473 */ }
/* FP:lib.rs-2474 */ 
/* FP:lib.rs-2475 */ #[derive(Copy, Clone, Debug)]
/* FP:lib.rs-2476 */ struct Finalize {
/* FP:lib.rs-2477 */     /// Node ID for linting.
/* FP:lib.rs-2478 */     node_id: NodeId,
/* FP:lib.rs-2479 */     /// Span of the whole path or some its characteristic fragment.
/* FP:lib.rs-2480 */     /// E.g. span of `b` in `foo::{a, b, c}`, or full span for regular paths.
/* FP:lib.rs-2481 */     path_span: Span,
/* FP:lib.rs-2482 */     /// Span of the path start, suitable for prepending something to it.
/* FP:lib.rs-2483 */     /// E.g. span of `foo` in `foo::{a, b, c}`, or full span for regular paths.
/* FP:lib.rs-2484 */     root_span: Span,
/* FP:lib.rs-2485 */     /// Whether to report privacy errors or silently return "no resolution" for them,
/* FP:lib.rs-2486 */     /// similarly to speculative resolution.
/* FP:lib.rs-2487 */     report_private: bool = true,
/* FP:lib.rs-2488 */     /// Tracks whether an item is used in scope or used relatively to a module.
/* FP:lib.rs-2489 */     used: Used = Used::Other,
/* FP:lib.rs-2490 */     /// Finalizing early or late resolution.
/* FP:lib.rs-2491 */     stage: Stage = Stage::Early,
/* FP:lib.rs-2492 */ }
/* FP:lib.rs-2493 */ 
/* FP:lib.rs-2494 */ impl Finalize {
/* FP:lib.rs-2495 */     fn new(node_id: NodeId, path_span: Span) -> Finalize {
/* FP:lib.rs-2496 */         Finalize::with_root_span(node_id, path_span, path_span)
/* FP:lib.rs-2497 */     }
/* FP:lib.rs-2498 */ 
/* FP:lib.rs-2499 */     fn with_root_span(node_id: NodeId, path_span: Span, root_span: Span) -> Finalize {
/* FP:lib.rs-2500 */         Finalize { node_id, path_span, root_span, .. }
/* FP:lib.rs-2501 */     }
/* FP:lib.rs-2502 */ }
/* FP:lib.rs-2503 */ 
/* FP:lib.rs-2504 */ pub fn provide(providers: &mut Providers) {
/* FP:lib.rs-2505 */     providers.registered_tools = macros::registered_tools;
/* FP:lib.rs-2506 */ }
/* FP:lib.rs-2507 */ 
/* FP:lib.rs-2508 */ mod ref_mut {
/* FP:lib.rs-2509 */     use std::ops::Deref;
/* FP:lib.rs-2510 */ 
/* FP:lib.rs-2511 */     /// A wrapper around a mutable reference that conditionally allows mutable access.
/* FP:lib.rs-2512 */     pub(crate) struct RefOrMut<'a, T> {
/* FP:lib.rs-2513 */         p: &'a mut T,
/* FP:lib.rs-2514 */         mutable: bool,
/* FP:lib.rs-2515 */     }
/* FP:lib.rs-2516 */ 
/* FP:lib.rs-2517 */     impl<'a, T> Deref for RefOrMut<'a, T> {
/* FP:lib.rs-2518 */         type Target = T;
/* FP:lib.rs-2519 */ 
/* FP:lib.rs-2520 */         fn deref(&self) -> &Self::Target {
/* FP:lib.rs-2521 */             self.p
/* FP:lib.rs-2522 */         }
/* FP:lib.rs-2523 */     }
/* FP:lib.rs-2524 */ 
/* FP:lib.rs-2525 */     impl<'a, T> AsRef<T> for RefOrMut<'a, T> {
/* FP:lib.rs-2526 */         fn as_ref(&self) -> &T {
/* FP:lib.rs-2527 */             self.p
/* FP:lib.rs-2528 */         }
/* FP:lib.rs-2529 */     }
/* FP:lib.rs-2530 */ 
/* FP:lib.rs-2531 */     impl<'a, T> RefOrMut<'a, T> {
/* FP:lib.rs-2532 */         pub(crate) fn new(p: &'a mut T, mutable: bool) -> Self {
/* FP:lib.rs-2533 */             RefOrMut { p, mutable }
/* FP:lib.rs-2534 */         }
/* FP:lib.rs-2535 */ 
/* FP:lib.rs-2536 */         /// This is needed because this wraps a `&mut T` and is therefore not `Copy`.
/* FP:lib.rs-2537 */         pub(crate) fn reborrow(&mut self) -> RefOrMut<'_, T> {
/* FP:lib.rs-2538 */             RefOrMut { p: self.p, mutable: self.mutable }
/* FP:lib.rs-2539 */         }
/* FP:lib.rs-2540 */ 
/* FP:lib.rs-2541 */         /// Returns a mutable reference to the inner value if allowed.
/* FP:lib.rs-2542 */         ///
/* FP:lib.rs-2543 */         /// # Panics
/* FP:lib.rs-2544 */         /// Panics if the `mutable` flag is false.
/* FP:lib.rs-2545 */         #[track_caller]
/* FP:lib.rs-2546 */         pub(crate) fn get_mut(&mut self) -> &mut T {
/* FP:lib.rs-2547 */             match self.mutable {
/* FP:lib.rs-2548 */                 false => panic!("Can't mutably borrow speculative resolver"),
/* FP:lib.rs-2549 */                 true => self.p,
/* FP:lib.rs-2550 */             }
/* FP:lib.rs-2551 */         }
/* FP:lib.rs-2552 */ 
/* FP:lib.rs-2553 */         /// Returns a mutable reference to the inner value without checking if
/* FP:lib.rs-2554 */         /// it's in a mutable state.
/* FP:lib.rs-2555 */         pub(crate) fn get_mut_unchecked(&mut self) -> &mut T {
/* FP:lib.rs-2556 */             self.p
/* FP:lib.rs-2557 */         }
/* FP:lib.rs-2558 */     }
/* FP:lib.rs-2559 */ }
/* FP:lib.rs-2560 */ 
/* FP:lib.rs-2561 */ /// A wrapper around `&mut Resolver` that may be mutable or immutable, depending on a conditions.
/* FP:lib.rs-2562 */ ///
/* FP:lib.rs-2563 */ /// `Cm` stands for "conditionally mutable".
/* FP:lib.rs-2564 */ ///
/* FP:lib.rs-2565 */ /// Prefer constructing it through [`Resolver::cm`] to ensure correctness.
/* FP:lib.rs-2566 */ type CmResolver<'r, 'ra, 'tcx> = ref_mut::RefOrMut<'r, Resolver<'ra, 'tcx>>;