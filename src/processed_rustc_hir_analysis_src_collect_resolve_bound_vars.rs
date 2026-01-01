/* FP:resolve_bound_vars.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_hir_analysis_src_collect_resolve_bound_vars_UNPARSEABLE_0001
/* FP:resolve_bound_vars.rs-0002 */ // Resolution of early vs late bound lifetimes.
/* FP:resolve_bound_vars.rs-0003 */ //
/* FP:resolve_bound_vars.rs-0004 */ // Name resolution for lifetimes is performed on the AST and embedded into HIR. From this
/* FP:resolve_bound_vars.rs-0005 */ // information, typechecking needs to transform the lifetime parameters into bound lifetimes.
/* FP:resolve_bound_vars.rs-0006 */ // Lifetimes can be early-bound or late-bound. Construction of typechecking terms needs to visit
/* FP:resolve_bound_vars.rs-0007 */ // the types in HIR to identify late-bound lifetimes and assign their Debruijn indices. This file
/* FP:resolve_bound_vars.rs-0008 */ // is also responsible for assigning their semantics to implicit lifetimes in trait objects.
/* FP:resolve_bound_vars.rs-0009 */ 
/* FP:resolve_bound_vars.rs-0010 */ use std::cell::RefCell;
/* FP:resolve_bound_vars.rs-0011 */ use std::fmt;
/* FP:resolve_bound_vars.rs-0012 */ use std::ops::ControlFlow;
/* FP:resolve_bound_vars.rs-0013 */ 
/* FP:resolve_bound_vars.rs-0014 */ use crate::rustc_complete::visit::walk_list;
/* FP:resolve_bound_vars.rs-0015 */ use crate::rustc_data_structures::fx::{FxHashSet, FxIndexMap, FxIndexSet};
/* FP:resolve_bound_vars.rs-0016 */ use crate::rustc_complete::ErrorGuaranteed;
/* FP:resolve_bound_vars.rs-0017 */ use crate::rustc_complete::def::{DefKind, Res};
/* FP:resolve_bound_vars.rs-0018 */ use crate::rustc_complete::definitions::{DefPathData, DisambiguatorState};
/* FP:resolve_bound_vars.rs-0019 */ use crate::rustc_complete::intravisit::{self, InferKind, Visitor, VisitorExt};
/* FP:resolve_bound_vars.rs-0020 */ use crate::rustc_complete::{
/* FP:resolve_bound_vars.rs-0021 */     self as hir, AmbigArg, GenericArg, GenericParam, GenericParamKind, HirId, LifetimeKind, Node,
/* FP:resolve_bound_vars.rs-0022 */ };
/* FP:resolve_bound_vars.rs-0023 */ use rustc_macros::extension;
/* FP:resolve_bound_vars.rs-0024 */ use crate::rustc_complete::hir::nested_filter;
/* FP:resolve_bound_vars.rs-0025 */ use crate::rustc_complete::middle::resolve_bound_vars::*;
/* FP:resolve_bound_vars.rs-0026 */ use crate::rustc_complete::query::Providers;
/* FP:resolve_bound_vars.rs-0027 */ use crate::rustc_complete::ty::{self, TyCtxt, TypeSuperVisitable, TypeVisitor};
/* FP:resolve_bound_vars.rs-0028 */ use crate::rustc_complete::{bug, span_bug};
/* FP:resolve_bound_vars.rs-0029 */ use crate::rustc_complete::def_id::{DefId, LocalDefId};
/* FP:resolve_bound_vars.rs-0030 */ use crate::rustc_complete::{Ident, Span, sym};
/* FP:resolve_bound_vars.rs-0031 */ use tracing::{debug, debug_span, instrument};
/* FP:resolve_bound_vars.rs-0032 */ 
/* FP:resolve_bound_vars.rs-0033 */ use crate::errors;
/* FP:resolve_bound_vars.rs-0034 */ 
/* FP:resolve_bound_vars.rs-0035 */ #[extension(trait RegionExt)]
/* FP:resolve_bound_vars.rs-0036 */ impl ResolvedArg {
/* FP:resolve_bound_vars.rs-0037 */     fn early(param: &GenericParam<'_>) -> ResolvedArg {
/* FP:resolve_bound_vars.rs-0038 */         ResolvedArg::EarlyBound(param.def_id)
/* FP:resolve_bound_vars.rs-0039 */     }
/* FP:resolve_bound_vars.rs-0040 */ 
/* FP:resolve_bound_vars.rs-0041 */     fn late(idx: u32, param: &GenericParam<'_>) -> ResolvedArg {
/* FP:resolve_bound_vars.rs-0042 */         ResolvedArg::LateBound(ty::INNERMOST, idx, param.def_id)
/* FP:resolve_bound_vars.rs-0043 */     }
/* FP:resolve_bound_vars.rs-0044 */ 
/* FP:resolve_bound_vars.rs-0045 */     fn id(&self) -> Option<LocalDefId> {
/* FP:resolve_bound_vars.rs-0046 */         match *self {
/* FP:resolve_bound_vars.rs-0047 */             ResolvedArg::StaticLifetime | ResolvedArg::Error(_) => None,
/* FP:resolve_bound_vars.rs-0048 */ 
/* FP:resolve_bound_vars.rs-0049 */             ResolvedArg::EarlyBound(id)
/* FP:resolve_bound_vars.rs-0050 */             | ResolvedArg::LateBound(_, _, id)
/* FP:resolve_bound_vars.rs-0051 */             | ResolvedArg::Free(_, id) => Some(id),
/* FP:resolve_bound_vars.rs-0052 */         }
/* FP:resolve_bound_vars.rs-0053 */     }
/* FP:resolve_bound_vars.rs-0054 */ 
/* FP:resolve_bound_vars.rs-0055 */     fn shifted(self, amount: u32) -> ResolvedArg {
/* FP:resolve_bound_vars.rs-0056 */         match self {
/* FP:resolve_bound_vars.rs-0057 */             ResolvedArg::LateBound(debruijn, idx, id) => {
/* FP:resolve_bound_vars.rs-0058 */                 ResolvedArg::LateBound(debruijn.shifted_in(amount), idx, id)
/* FP:resolve_bound_vars.rs-0059 */             }
/* FP:resolve_bound_vars.rs-0060 */             _ => self,
/* FP:resolve_bound_vars.rs-0061 */         }
/* FP:resolve_bound_vars.rs-0062 */     }
/* FP:resolve_bound_vars.rs-0063 */ }
/* FP:resolve_bound_vars.rs-0064 */ 
/* FP:resolve_bound_vars.rs-0065 */ struct BoundVarContext<'a, 'tcx> {
/* FP:resolve_bound_vars.rs-0066 */     tcx: TyCtxt<'tcx>,
/* FP:resolve_bound_vars.rs-0067 */     rbv: &'a mut ResolveBoundVars,
/* FP:resolve_bound_vars.rs-0068 */     disambiguator: &'a mut DisambiguatorState,
/* FP:resolve_bound_vars.rs-0069 */     scope: ScopeRef<'a>,
/* FP:resolve_bound_vars.rs-0070 */ }
/* FP:resolve_bound_vars.rs-0071 */ 
/* FP:resolve_bound_vars.rs-0072 */ #[derive(Debug)]
/* FP:resolve_bound_vars.rs-0073 */ enum Scope<'a> {
/* FP:resolve_bound_vars.rs-0074 */     /// Declares lifetimes, and each can be early-bound or late-bound.
/* FP:resolve_bound_vars.rs-0075 */     /// The `DebruijnIndex` of late-bound lifetimes starts at `1` and
/* FP:resolve_bound_vars.rs-0076 */     /// it should be shifted by the number of `Binder`s in between the
/* FP:resolve_bound_vars.rs-0077 */     /// declaration `Binder` and the location it's referenced from.
/* FP:resolve_bound_vars.rs-0078 */     Binder {
/* FP:resolve_bound_vars.rs-0079 */         /// We use an IndexMap here because we want these lifetimes in order
/* FP:resolve_bound_vars.rs-0080 */         /// for diagnostics.
/* FP:resolve_bound_vars.rs-0081 */         bound_vars: FxIndexMap<LocalDefId, ResolvedArg>,
/* FP:resolve_bound_vars.rs-0082 */ 
/* FP:resolve_bound_vars.rs-0083 */         scope_type: BinderScopeType,
/* FP:resolve_bound_vars.rs-0084 */ 
/* FP:resolve_bound_vars.rs-0085 */         /// The late bound vars for a given item are stored by `HirId` to be
/* FP:resolve_bound_vars.rs-0086 */         /// queried later. However, if we enter an elision scope, we have to
/* FP:resolve_bound_vars.rs-0087 */         /// later append the elided bound vars to the list and need to know what
/* FP:resolve_bound_vars.rs-0088 */         /// to append to.
/* FP:resolve_bound_vars.rs-0089 */         hir_id: HirId,
/* FP:resolve_bound_vars.rs-0090 */ 
/* FP:resolve_bound_vars.rs-0091 */         s: ScopeRef<'a>,
/* FP:resolve_bound_vars.rs-0092 */ 
/* FP:resolve_bound_vars.rs-0093 */         /// If this binder comes from a where clause, specify how it was created.
/* FP:resolve_bound_vars.rs-0094 */         /// This is used to diagnose inaccessible lifetimes in APIT:
/* FP:resolve_bound_vars.rs-0095 */         /// ```ignore (illustrative)
/* FP:resolve_bound_vars.rs-0096 */         /// fn foo(x: impl for<'a> Trait<'a, Assoc = impl Copy + 'a>) {}
/* FP:resolve_bound_vars.rs-0097 */         /// ```
/* FP:resolve_bound_vars.rs-0098 */         where_bound_origin: Option<hir::PredicateOrigin>,
/* FP:resolve_bound_vars.rs-0099 */     },
/* FP:resolve_bound_vars.rs-0100 */ 
/* FP:resolve_bound_vars.rs-0101 */     /// Lifetimes introduced by a fn are scoped to the call-site for that fn,
/* FP:resolve_bound_vars.rs-0102 */     /// if this is a fn body, otherwise the original definitions are used.
/* FP:resolve_bound_vars.rs-0103 */     /// Unspecified lifetimes are inferred, unless an elision scope is nested,
/* FP:resolve_bound_vars.rs-0104 */     /// e.g., `(&T, fn(&T) -> &T);` becomes `(&'_ T, for<'a> fn(&'a T) -> &'a T)`.
/* FP:resolve_bound_vars.rs-0105 */     Body {
/* FP:resolve_bound_vars.rs-0106 */         id: hir::BodyId,
/* FP:resolve_bound_vars.rs-0107 */         s: ScopeRef<'a>,
/* FP:resolve_bound_vars.rs-0108 */     },
/* FP:resolve_bound_vars.rs-0109 */ 
/* FP:resolve_bound_vars.rs-0110 */     /// Use a specific lifetime (if `Some`) or leave it unset (to be
/* FP:resolve_bound_vars.rs-0111 */     /// inferred in a function body or potentially error outside one),
/* FP:resolve_bound_vars.rs-0112 */     /// for the default choice of lifetime in a trait object type.
/* FP:resolve_bound_vars.rs-0113 */     ObjectLifetimeDefault {
/* FP:resolve_bound_vars.rs-0114 */         lifetime: Option<ResolvedArg>,
/* FP:resolve_bound_vars.rs-0115 */         s: ScopeRef<'a>,
/* FP:resolve_bound_vars.rs-0116 */     },
/* FP:resolve_bound_vars.rs-0117 */ 
/* FP:resolve_bound_vars.rs-0118 */     /// When we have nested trait refs, we concatenate late bound vars for inner
/* FP:resolve_bound_vars.rs-0119 */     /// trait refs from outer ones. But we also need to include any HRTB
/* FP:resolve_bound_vars.rs-0120 */     /// lifetimes encountered when identifying the trait that an associated type
/* FP:resolve_bound_vars.rs-0121 */     /// is declared on.
/* FP:resolve_bound_vars.rs-0122 */     Supertrait {
/* FP:resolve_bound_vars.rs-0123 */         bound_vars: Vec<ty::BoundVariableKind>,
/* FP:resolve_bound_vars.rs-0124 */         s: ScopeRef<'a>,
/* FP:resolve_bound_vars.rs-0125 */     },
/* FP:resolve_bound_vars.rs-0126 */ 
/* FP:resolve_bound_vars.rs-0127 */     TraitRefBoundary {
/* FP:resolve_bound_vars.rs-0128 */         s: ScopeRef<'a>,
/* FP:resolve_bound_vars.rs-0129 */     },
/* FP:resolve_bound_vars.rs-0130 */ 
/* FP:resolve_bound_vars.rs-0131 */     /// Remap lifetimes that appear in opaque types to fresh lifetime parameters. Given:
/* FP:resolve_bound_vars.rs-0132 */     /// `fn foo<'a>() -> impl MyTrait<'a> { ... }`
/* FP:resolve_bound_vars.rs-0133 */     ///
/* FP:resolve_bound_vars.rs-0134 */     /// HIR tells us that `'a` refer to the lifetime bound on `foo`.
/* FP:resolve_bound_vars.rs-0135 */     /// However, typeck and borrowck for opaques work based on using a new generic type.
/* FP:resolve_bound_vars.rs-0136 */     /// `type MyAnonTy<'b> = impl MyTrait<'b>;`
/* FP:resolve_bound_vars.rs-0137 */     ///
/* FP:resolve_bound_vars.rs-0138 */     /// This scope collects the mapping `'a -> 'b`.
/* FP:resolve_bound_vars.rs-0139 */     Opaque {
/* FP:resolve_bound_vars.rs-0140 */         /// The opaque type we are traversing.
/* FP:resolve_bound_vars.rs-0141 */         def_id: LocalDefId,
/* FP:resolve_bound_vars.rs-0142 */         /// Mapping from each captured lifetime `'a` to the duplicate generic parameter `'b`.
/* FP:resolve_bound_vars.rs-0143 */         captures: &'a RefCell<FxIndexMap<ResolvedArg, LocalDefId>>,
/* FP:resolve_bound_vars.rs-0144 */ 
/* FP:resolve_bound_vars.rs-0145 */         s: ScopeRef<'a>,
/* FP:resolve_bound_vars.rs-0146 */     },
/* FP:resolve_bound_vars.rs-0147 */ 
/* FP:resolve_bound_vars.rs-0148 */     /// Disallows capturing late-bound vars from parent scopes.
/* FP:resolve_bound_vars.rs-0149 */     ///
/* FP:resolve_bound_vars.rs-0150 */     /// This is necessary for something like `for<T> [(); { /* references T */ }]:`,
/* FP:resolve_bound_vars.rs-0151 */     /// since we don't do something more correct like replacing any captured
/* FP:resolve_bound_vars.rs-0152 */     /// late-bound vars with early-bound params in the const's own generics.
/* FP:resolve_bound_vars.rs-0153 */     LateBoundary {
/* FP:resolve_bound_vars.rs-0154 */         s: ScopeRef<'a>,
/* FP:resolve_bound_vars.rs-0155 */         what: &'static str,
/* FP:resolve_bound_vars.rs-0156 */         deny_late_regions: bool,
/* FP:resolve_bound_vars.rs-0157 */     },
/* FP:resolve_bound_vars.rs-0158 */ 
/* FP:resolve_bound_vars.rs-0159 */     Root {
/* FP:resolve_bound_vars.rs-0160 */         opt_parent_item: Option<LocalDefId>,
/* FP:resolve_bound_vars.rs-0161 */     },
/* FP:resolve_bound_vars.rs-0162 */ }
/* FP:resolve_bound_vars.rs-0163 */ 
/* FP:resolve_bound_vars.rs-0164 */ impl<'a> Scope<'a> {
/* FP:resolve_bound_vars.rs-0165 */     // A helper for debugging scopes without printing parent scopes
/* FP:resolve_bound_vars.rs-0166 */     fn debug_truncated(&self) -> impl fmt::Debug {
/* FP:resolve_bound_vars.rs-0167 */         fmt::from_fn(move |f| match self {
/* FP:resolve_bound_vars.rs-0168 */             Self::Binder { bound_vars, scope_type, hir_id, where_bound_origin, s: _ } => f
/* FP:resolve_bound_vars.rs-0169 */                 .debug_struct("Binder")
/* FP:resolve_bound_vars.rs-0170 */                 .field("bound_vars", bound_vars)
/* FP:resolve_bound_vars.rs-0171 */                 .field("scope_type", scope_type)
/* FP:resolve_bound_vars.rs-0172 */                 .field("hir_id", hir_id)
/* FP:resolve_bound_vars.rs-0173 */                 .field("where_bound_origin", where_bound_origin)
/* FP:resolve_bound_vars.rs-0174 */                 .field("s", &"..")
/* FP:resolve_bound_vars.rs-0175 */                 .finish(),
/* FP:resolve_bound_vars.rs-0176 */             Self::Opaque { captures, def_id, s: _ } => f
/* FP:resolve_bound_vars.rs-0177 */                 .debug_struct("Opaque")
/* FP:resolve_bound_vars.rs-0178 */                 .field("def_id", def_id)
/* FP:resolve_bound_vars.rs-0179 */                 .field("captures", &captures.borrow())
/* FP:resolve_bound_vars.rs-0180 */                 .field("s", &"..")
/* FP:resolve_bound_vars.rs-0181 */                 .finish(),
/* FP:resolve_bound_vars.rs-0182 */             Self::Body { id, s: _ } => {
/* FP:resolve_bound_vars.rs-0183 */                 f.debug_struct("Body").field("id", id).field("s", &"..").finish()
/* FP:resolve_bound_vars.rs-0184 */             }
/* FP:resolve_bound_vars.rs-0185 */             Self::ObjectLifetimeDefault { lifetime, s: _ } => f
/* FP:resolve_bound_vars.rs-0186 */                 .debug_struct("ObjectLifetimeDefault")
/* FP:resolve_bound_vars.rs-0187 */                 .field("lifetime", lifetime)
/* FP:resolve_bound_vars.rs-0188 */                 .field("s", &"..")
/* FP:resolve_bound_vars.rs-0189 */                 .finish(),
/* FP:resolve_bound_vars.rs-0190 */             Self::Supertrait { bound_vars, s: _ } => f
/* FP:resolve_bound_vars.rs-0191 */                 .debug_struct("Supertrait")
/* FP:resolve_bound_vars.rs-0192 */                 .field("bound_vars", bound_vars)
/* FP:resolve_bound_vars.rs-0193 */                 .field("s", &"..")
/* FP:resolve_bound_vars.rs-0194 */                 .finish(),
/* FP:resolve_bound_vars.rs-0195 */             Self::TraitRefBoundary { s: _ } => f.debug_struct("TraitRefBoundary").finish(),
/* FP:resolve_bound_vars.rs-0196 */             Self::LateBoundary { s: _, what, deny_late_regions } => f
/* FP:resolve_bound_vars.rs-0197 */                 .debug_struct("LateBoundary")
/* FP:resolve_bound_vars.rs-0198 */                 .field("what", what)
/* FP:resolve_bound_vars.rs-0199 */                 .field("deny_late_regions", deny_late_regions)
/* FP:resolve_bound_vars.rs-0200 */                 .finish(),
/* FP:resolve_bound_vars.rs-0201 */             Self::Root { opt_parent_item } => {
/* FP:resolve_bound_vars.rs-0202 */                 f.debug_struct("Root").field("opt_parent_item", &opt_parent_item).finish()
/* FP:resolve_bound_vars.rs-0203 */             }
/* FP:resolve_bound_vars.rs-0204 */         })
/* FP:resolve_bound_vars.rs-0205 */     }
/* FP:resolve_bound_vars.rs-0206 */ }
/* FP:resolve_bound_vars.rs-0207 */ 
/* FP:resolve_bound_vars.rs-0208 */ #[derive(Copy, Clone, Debug)]
/* FP:resolve_bound_vars.rs-0209 */ enum BinderScopeType {
/* FP:resolve_bound_vars.rs-0210 */     /// Any non-concatenating binder scopes.
/* FP:resolve_bound_vars.rs-0211 */     Normal,
/* FP:resolve_bound_vars.rs-0212 */     /// Within a syntactic trait ref, there may be multiple poly trait refs that
/* FP:resolve_bound_vars.rs-0213 */     /// are nested (under the `associated_type_bounds` feature). The binders of
/* FP:resolve_bound_vars.rs-0214 */     /// the inner poly trait refs are extended from the outer poly trait refs
/* FP:resolve_bound_vars.rs-0215 */     /// and don't increase the late bound depth. If you had
/* FP:resolve_bound_vars.rs-0216 */     /// `T: for<'a>  Foo<Bar: for<'b> Baz<'a, 'b>>`, then the `for<'b>` scope
/* FP:resolve_bound_vars.rs-0217 */     /// would be `Concatenating`. This also used in trait refs in where clauses
/* FP:resolve_bound_vars.rs-0218 */     /// where we have two binders `for<> T: for<> Foo` (I've intentionally left
/* FP:resolve_bound_vars.rs-0219 */     /// out any lifetimes because they aren't needed to show the two scopes).
/* FP:resolve_bound_vars.rs-0220 */     /// The inner `for<>` has a scope of `Concatenating`.
/* FP:resolve_bound_vars.rs-0221 */     Concatenating,
/* FP:resolve_bound_vars.rs-0222 */ }
/* FP:resolve_bound_vars.rs-0223 */ 
/* FP:resolve_bound_vars.rs-0224 */ type ScopeRef<'a> = &'a Scope<'a>;
/* FP:resolve_bound_vars.rs-0225 */ 
/* FP:resolve_bound_vars.rs-0226 */ /// Adds query implementations to the [Providers] vtable, see [`crate::rustc_middle::query`]
/* FP:resolve_bound_vars.rs-0227 */ pub(crate) fn provide(providers: &mut Providers) {
/* FP:resolve_bound_vars.rs-0228 */     *providers = Providers {
/* FP:resolve_bound_vars.rs-0229 */         resolve_bound_vars,
/* FP:resolve_bound_vars.rs-0230 */ 
/* FP:resolve_bound_vars.rs-0231 */         named_variable_map: |tcx, id| &tcx.resolve_bound_vars(id).defs,
/* FP:resolve_bound_vars.rs-0232 */         is_late_bound_map,
/* FP:resolve_bound_vars.rs-0233 */         object_lifetime_default,
/* FP:resolve_bound_vars.rs-0234 */         late_bound_vars_map: |tcx, id| &tcx.resolve_bound_vars(id).late_bound_vars,
/* FP:resolve_bound_vars.rs-0235 */         opaque_captured_lifetimes: |tcx, id| {
/* FP:resolve_bound_vars.rs-0236 */             &tcx.resolve_bound_vars(tcx.local_def_id_to_hir_id(id).owner)
/* FP:resolve_bound_vars.rs-0237 */                 .opaque_captured_lifetimes
/* FP:resolve_bound_vars.rs-0238 */                 .get(&id)
/* FP:resolve_bound_vars.rs-0239 */                 .map_or(&[][..], |x| &x[..])
/* FP:resolve_bound_vars.rs-0240 */         },
/* FP:resolve_bound_vars.rs-0241 */ 
/* FP:resolve_bound_vars.rs-0242 */         ..*providers
/* FP:resolve_bound_vars.rs-0243 */     };
/* FP:resolve_bound_vars.rs-0244 */ }
/* FP:resolve_bound_vars.rs-0245 */ 
/* FP:resolve_bound_vars.rs-0246 */ /// Computes the `ResolveBoundVars` map that contains data for an entire `Item`.
/* FP:resolve_bound_vars.rs-0247 */ /// You should not read the result of this query directly, but rather use
/* FP:resolve_bound_vars.rs-0248 */ /// `named_variable_map`, `late_bound_vars_map`, etc.
/* FP:resolve_bound_vars.rs-0249 */ #[instrument(level = "debug", skip(tcx))]
/* FP:resolve_bound_vars.rs-0250 */ fn resolve_bound_vars(tcx: TyCtxt<'_>, local_def_id: hir::OwnerId) -> ResolveBoundVars {
/* FP:resolve_bound_vars.rs-0251 */     let mut rbv = ResolveBoundVars::default();
/* FP:resolve_bound_vars.rs-0252 */     let mut visitor = BoundVarContext {
/* FP:resolve_bound_vars.rs-0253 */         tcx,
/* FP:resolve_bound_vars.rs-0254 */         rbv: &mut rbv,
/* FP:resolve_bound_vars.rs-0255 */         scope: &Scope::Root { opt_parent_item: None },
/* FP:resolve_bound_vars.rs-0256 */         disambiguator: &mut DisambiguatorState::new(),
/* FP:resolve_bound_vars.rs-0257 */     };
/* FP:resolve_bound_vars.rs-0258 */     match tcx.hir_owner_node(local_def_id) {
/* FP:resolve_bound_vars.rs-0259 */         hir::OwnerNode::Item(item) => visitor.visit_item(item),
/* FP:resolve_bound_vars.rs-0260 */         hir::OwnerNode::ForeignItem(item) => visitor.visit_foreign_item(item),
/* FP:resolve_bound_vars.rs-0261 */         hir::OwnerNode::TraitItem(item) => {
/* FP:resolve_bound_vars.rs-0262 */             let scope =
/* FP:resolve_bound_vars.rs-0263 */                 Scope::Root { opt_parent_item: Some(tcx.local_parent(item.owner_id.def_id)) };
/* FP:resolve_bound_vars.rs-0264 */             visitor.scope = &scope;
/* FP:resolve_bound_vars.rs-0265 */             visitor.visit_trait_item(item)
/* FP:resolve_bound_vars.rs-0266 */         }
/* FP:resolve_bound_vars.rs-0267 */         hir::OwnerNode::ImplItem(item) => {
/* FP:resolve_bound_vars.rs-0268 */             let scope =
/* FP:resolve_bound_vars.rs-0269 */                 Scope::Root { opt_parent_item: Some(tcx.local_parent(item.owner_id.def_id)) };
/* FP:resolve_bound_vars.rs-0270 */             visitor.scope = &scope;
/* FP:resolve_bound_vars.rs-0271 */             visitor.visit_impl_item(item)
/* FP:resolve_bound_vars.rs-0272 */         }
/* FP:resolve_bound_vars.rs-0273 */         hir::OwnerNode::Crate(_) => {}
/* FP:resolve_bound_vars.rs-0274 */         hir::OwnerNode::Synthetic => unreachable!(),
/* FP:resolve_bound_vars.rs-0275 */     }
/* FP:resolve_bound_vars.rs-0276 */ 
/* FP:resolve_bound_vars.rs-0277 */     debug!(?rbv.defs);
/* FP:resolve_bound_vars.rs-0278 */     debug!(?rbv.late_bound_vars);
/* FP:resolve_bound_vars.rs-0279 */     debug!(?rbv.opaque_captured_lifetimes);
/* FP:resolve_bound_vars.rs-0280 */     rbv
/* FP:resolve_bound_vars.rs-0281 */ }
/* FP:resolve_bound_vars.rs-0282 */ 
/* FP:resolve_bound_vars.rs-0283 */ fn late_arg_as_bound_arg<'tcx>(param: &GenericParam<'tcx>) -> ty::BoundVariableKind {
/* FP:resolve_bound_vars.rs-0284 */     let def_id = param.def_id.to_def_id();
/* FP:resolve_bound_vars.rs-0285 */     match param.kind {
/* FP:resolve_bound_vars.rs-0286 */         GenericParamKind::Lifetime { .. } => {
/* FP:resolve_bound_vars.rs-0287 */             ty::BoundVariableKind::Region(ty::BoundRegionKind::Named(def_id))
/* FP:resolve_bound_vars.rs-0288 */         }
/* FP:resolve_bound_vars.rs-0289 */         GenericParamKind::Type { .. } => ty::BoundVariableKind::Ty(ty::BoundTyKind::Param(def_id)),
/* FP:resolve_bound_vars.rs-0290 */         GenericParamKind::Const { .. } => ty::BoundVariableKind::Const,
/* FP:resolve_bound_vars.rs-0291 */     }
/* FP:resolve_bound_vars.rs-0292 */ }
/* FP:resolve_bound_vars.rs-0293 */ 
/* FP:resolve_bound_vars.rs-0294 */ /// Turn a [`ty::GenericParamDef`] into a bound arg. Generally, this should only
/* FP:resolve_bound_vars.rs-0295 */ /// be used when turning early-bound vars into late-bound vars when lowering
/* FP:resolve_bound_vars.rs-0296 */ /// return type notation.
/* FP:resolve_bound_vars.rs-0297 */ fn generic_param_def_as_bound_arg(param: &ty::GenericParamDef) -> ty::BoundVariableKind {
/* FP:resolve_bound_vars.rs-0298 */     match param.kind {
/* FP:resolve_bound_vars.rs-0299 */         ty::GenericParamDefKind::Lifetime => {
/* FP:resolve_bound_vars.rs-0300 */             ty::BoundVariableKind::Region(ty::BoundRegionKind::Named(param.def_id))
/* FP:resolve_bound_vars.rs-0301 */         }
/* FP:resolve_bound_vars.rs-0302 */         ty::GenericParamDefKind::Type { .. } => {
/* FP:resolve_bound_vars.rs-0303 */             ty::BoundVariableKind::Ty(ty::BoundTyKind::Param(param.def_id))
/* FP:resolve_bound_vars.rs-0304 */         }
/* FP:resolve_bound_vars.rs-0305 */         ty::GenericParamDefKind::Const { .. } => ty::BoundVariableKind::Const,
/* FP:resolve_bound_vars.rs-0306 */     }
/* FP:resolve_bound_vars.rs-0307 */ }
/* FP:resolve_bound_vars.rs-0308 */ 
/* FP:resolve_bound_vars.rs-0309 */ /// Whether this opaque always captures lifetimes in scope.
/* FP:resolve_bound_vars.rs-0310 */ /// Right now, this is all RPITIT and TAITs, and when the opaque
/* FP:resolve_bound_vars.rs-0311 */ /// is coming from a span corresponding to edition 2024.
/* FP:resolve_bound_vars.rs-0312 */ fn opaque_captures_all_in_scope_lifetimes<'tcx>(opaque: &'tcx hir::OpaqueTy<'tcx>) -> bool {
/* FP:resolve_bound_vars.rs-0313 */     match opaque.origin {
/* FP:resolve_bound_vars.rs-0314 */         // if the opaque has the `use<...>` syntax, the user is telling us that they only want
/* FP:resolve_bound_vars.rs-0315 */         // to account for those lifetimes, so do not try to be clever.
/* FP:resolve_bound_vars.rs-0316 */         _ if opaque.bounds.iter().any(|bound| matches!(bound, hir::GenericBound::Use(..))) => false,
/* FP:resolve_bound_vars.rs-0317 */         hir::OpaqueTyOrigin::AsyncFn { .. } | hir::OpaqueTyOrigin::TyAlias { .. } => true,
/* FP:resolve_bound_vars.rs-0318 */         _ if opaque.span.at_least_rust_2024() => true,
/* FP:resolve_bound_vars.rs-0319 */         hir::OpaqueTyOrigin::FnReturn { in_trait_or_impl, .. } => in_trait_or_impl.is_some(),
/* FP:resolve_bound_vars.rs-0320 */     }
/* FP:resolve_bound_vars.rs-0321 */ }
/* FP:resolve_bound_vars.rs-0322 */ 
/* FP:resolve_bound_vars.rs-0323 */ impl<'a, 'tcx> BoundVarContext<'a, 'tcx> {
/* FP:resolve_bound_vars.rs-0324 */     /// Returns the binders in scope and the type of `Binder` that should be created for a poly trait ref.
/* FP:resolve_bound_vars.rs-0325 */     fn poly_trait_ref_binder_info(&mut self) -> (Vec<ty::BoundVariableKind>, BinderScopeType) {
/* FP:resolve_bound_vars.rs-0326 */         let mut scope = self.scope;
/* FP:resolve_bound_vars.rs-0327 */         let mut supertrait_bound_vars = vec![];
/* FP:resolve_bound_vars.rs-0328 */         loop {
/* FP:resolve_bound_vars.rs-0329 */             match scope {
/* FP:resolve_bound_vars.rs-0330 */                 Scope::Body { .. } | Scope::Root { .. } => {
/* FP:resolve_bound_vars.rs-0331 */                     break (vec![], BinderScopeType::Normal);
/* FP:resolve_bound_vars.rs-0332 */                 }
/* FP:resolve_bound_vars.rs-0333 */ 
/* FP:resolve_bound_vars.rs-0334 */                 Scope::Opaque { s, .. }
/* FP:resolve_bound_vars.rs-0335 */                 | Scope::ObjectLifetimeDefault { s, .. }
/* FP:resolve_bound_vars.rs-0336 */                 | Scope::LateBoundary { s, .. } => {
/* FP:resolve_bound_vars.rs-0337 */                     scope = s;
/* FP:resolve_bound_vars.rs-0338 */                 }
/* FP:resolve_bound_vars.rs-0339 */ 
/* FP:resolve_bound_vars.rs-0340 */                 Scope::Supertrait { s, bound_vars } => {
/* FP:resolve_bound_vars.rs-0341 */                     supertrait_bound_vars = bound_vars.clone();
/* FP:resolve_bound_vars.rs-0342 */                     scope = s;
/* FP:resolve_bound_vars.rs-0343 */                 }
/* FP:resolve_bound_vars.rs-0344 */ 
/* FP:resolve_bound_vars.rs-0345 */                 Scope::TraitRefBoundary { .. } => {
/* FP:resolve_bound_vars.rs-0346 */                     // We should only see super trait lifetimes if there is a `Binder` above
/* FP:resolve_bound_vars.rs-0347 */                     // though this may happen when we call `poly_trait_ref_binder_info` with
/* FP:resolve_bound_vars.rs-0348 */                     // an (erroneous, #113423) associated return type bound in an impl header.
/* FP:resolve_bound_vars.rs-0349 */                     if !supertrait_bound_vars.is_empty() {
/* FP:resolve_bound_vars.rs-0350 */                         self.tcx.dcx().delayed_bug(format!(
/* FP:resolve_bound_vars.rs-0351 */                             "found supertrait lifetimes without a binder to append \
/* FP:resolve_bound_vars.rs-0352 */                                 them to: {supertrait_bound_vars:?}"
/* FP:resolve_bound_vars.rs-0353 */                         ));
/* FP:resolve_bound_vars.rs-0354 */                     }
/* FP:resolve_bound_vars.rs-0355 */                     break (vec![], BinderScopeType::Normal);
/* FP:resolve_bound_vars.rs-0356 */                 }
/* FP:resolve_bound_vars.rs-0357 */ 
/* FP:resolve_bound_vars.rs-0358 */                 Scope::Binder { hir_id, .. } => {
/* FP:resolve_bound_vars.rs-0359 */                     // Nested poly trait refs have the binders concatenated
/* FP:resolve_bound_vars.rs-0360 */                     let mut full_binders =
/* FP:resolve_bound_vars.rs-0361 */                         self.rbv.late_bound_vars.get_mut_or_insert_default(hir_id.local_id).clone();
/* FP:resolve_bound_vars.rs-0362 */                     full_binders.extend(supertrait_bound_vars);
/* FP:resolve_bound_vars.rs-0363 */                     break (full_binders, BinderScopeType::Concatenating);
/* FP:resolve_bound_vars.rs-0364 */                 }
/* FP:resolve_bound_vars.rs-0365 */             }
/* FP:resolve_bound_vars.rs-0366 */         }
/* FP:resolve_bound_vars.rs-0367 */     }
/* FP:resolve_bound_vars.rs-0368 */ 
/* FP:resolve_bound_vars.rs-0369 */     fn visit_poly_trait_ref_inner(
/* FP:resolve_bound_vars.rs-0370 */         &mut self,
/* FP:resolve_bound_vars.rs-0371 */         trait_ref: &'tcx hir::PolyTraitRef<'tcx>,
/* FP:resolve_bound_vars.rs-0372 */         non_lifetime_binder_allowed: NonLifetimeBinderAllowed,
/* FP:resolve_bound_vars.rs-0373 */     ) {
/* FP:resolve_bound_vars.rs-0374 */         debug!("visit_poly_trait_ref(trait_ref={:?})", trait_ref);
/* FP:resolve_bound_vars.rs-0375 */ 
/* FP:resolve_bound_vars.rs-0376 */         let (mut binders, scope_type) = self.poly_trait_ref_binder_info();
/* FP:resolve_bound_vars.rs-0377 */ 
/* FP:resolve_bound_vars.rs-0378 */         let initial_bound_vars = binders.len() as u32;
/* FP:resolve_bound_vars.rs-0379 */         let mut bound_vars: FxIndexMap<LocalDefId, ResolvedArg> = FxIndexMap::default();
/* FP:resolve_bound_vars.rs-0380 */         let binders_iter =
/* FP:resolve_bound_vars.rs-0381 */             trait_ref.bound_generic_params.iter().enumerate().map(|(late_bound_idx, param)| {
/* FP:resolve_bound_vars.rs-0382 */                 let arg = ResolvedArg::late(initial_bound_vars + late_bound_idx as u32, param);
/* FP:resolve_bound_vars.rs-0383 */                 bound_vars.insert(param.def_id, arg);
/* FP:resolve_bound_vars.rs-0384 */                 late_arg_as_bound_arg(param)
/* FP:resolve_bound_vars.rs-0385 */             });
/* FP:resolve_bound_vars.rs-0386 */         binders.extend(binders_iter);
/* FP:resolve_bound_vars.rs-0387 */ 
/* FP:resolve_bound_vars.rs-0388 */         if let NonLifetimeBinderAllowed::Deny(where_) = non_lifetime_binder_allowed {
/* FP:resolve_bound_vars.rs-0389 */             deny_non_region_late_bound(self.tcx, &mut bound_vars, where_);
/* FP:resolve_bound_vars.rs-0390 */         }
/* FP:resolve_bound_vars.rs-0391 */ 
/* FP:resolve_bound_vars.rs-0392 */         debug!(?binders);
/* FP:resolve_bound_vars.rs-0393 */         self.record_late_bound_vars(trait_ref.trait_ref.hir_ref_id, binders);
/* FP:resolve_bound_vars.rs-0394 */ 
/* FP:resolve_bound_vars.rs-0395 */         // Always introduce a scope here, even if this is in a where clause and
/* FP:resolve_bound_vars.rs-0396 */         // we introduced the binders around the bounded Ty. In that case, we
/* FP:resolve_bound_vars.rs-0397 */         // just reuse the concatenation functionality also present in nested trait
/* FP:resolve_bound_vars.rs-0398 */         // refs.
/* FP:resolve_bound_vars.rs-0399 */         let scope = Scope::Binder {
/* FP:resolve_bound_vars.rs-0400 */             hir_id: trait_ref.trait_ref.hir_ref_id,
/* FP:resolve_bound_vars.rs-0401 */             bound_vars,
/* FP:resolve_bound_vars.rs-0402 */             s: self.scope,
/* FP:resolve_bound_vars.rs-0403 */             scope_type,
/* FP:resolve_bound_vars.rs-0404 */             where_bound_origin: None,
/* FP:resolve_bound_vars.rs-0405 */         };
/* FP:resolve_bound_vars.rs-0406 */         self.with(scope, |this| {
/* FP:resolve_bound_vars.rs-0407 */             walk_list!(this, visit_generic_param, trait_ref.bound_generic_params);
/* FP:resolve_bound_vars.rs-0408 */             this.visit_trait_ref(&trait_ref.trait_ref);
/* FP:resolve_bound_vars.rs-0409 */         });
/* FP:resolve_bound_vars.rs-0410 */     }
/* FP:resolve_bound_vars.rs-0411 */ }
/* FP:resolve_bound_vars.rs-0412 */ 
/* FP:resolve_bound_vars.rs-0413 */ enum NonLifetimeBinderAllowed {
/* FP:resolve_bound_vars.rs-0414 */     Deny(&'static str),
/* FP:resolve_bound_vars.rs-0415 */     Allow,
/* FP:resolve_bound_vars.rs-0416 */ }
/* FP:resolve_bound_vars.rs-0417 */ 
/* FP:resolve_bound_vars.rs-0418 */ impl<'a, 'tcx> Visitor<'tcx> for BoundVarContext<'a, 'tcx> {
/* FP:resolve_bound_vars.rs-0419 */     type NestedFilter = nested_filter::OnlyBodies;
/* FP:resolve_bound_vars.rs-0420 */ 
/* FP:resolve_bound_vars.rs-0421 */     fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
/* FP:resolve_bound_vars.rs-0422 */         self.tcx
/* FP:resolve_bound_vars.rs-0423 */     }
/* FP:resolve_bound_vars.rs-0424 */ 
/* FP:resolve_bound_vars.rs-0425 */     fn visit_nested_body(&mut self, body: hir::BodyId) {
/* FP:resolve_bound_vars.rs-0426 */         let body = self.tcx.hir_body(body);
/* FP:resolve_bound_vars.rs-0427 */         self.with(Scope::Body { id: body.id(), s: self.scope }, |this| {
/* FP:resolve_bound_vars.rs-0428 */             this.visit_body(body);
/* FP:resolve_bound_vars.rs-0429 */         });
/* FP:resolve_bound_vars.rs-0430 */     }
/* FP:resolve_bound_vars.rs-0431 */ 
/* FP:resolve_bound_vars.rs-0432 */     fn visit_expr(&mut self, e: &'tcx hir::Expr<'tcx>) {
/* FP:resolve_bound_vars.rs-0433 */         if let hir::ExprKind::Closure(hir::Closure {
/* FP:resolve_bound_vars.rs-0434 */             binder, bound_generic_params, fn_decl, ..
/* FP:resolve_bound_vars.rs-0435 */         }) = e.kind
/* FP:resolve_bound_vars.rs-0436 */         {
/* FP:resolve_bound_vars.rs-0437 */             if let &hir::ClosureBinder::For { span: for_sp, .. } = binder {
/* FP:resolve_bound_vars.rs-0438 */                 fn span_of_infer(ty: &hir::Ty<'_>) -> Option<Span> {
/* FP:resolve_bound_vars.rs-0439 */                     /// Look for `_` anywhere in the signature of a `for<> ||` closure.
/* FP:resolve_bound_vars.rs-0440 */                     /// This is currently disallowed.
/* FP:resolve_bound_vars.rs-0441 */                     struct FindInferInClosureWithBinder;
/* FP:resolve_bound_vars.rs-0442 */                     impl<'v> Visitor<'v> for FindInferInClosureWithBinder {
/* FP:resolve_bound_vars.rs-0443 */                         type Result = ControlFlow<Span>;
/* FP:resolve_bound_vars.rs-0444 */ 
/* FP:resolve_bound_vars.rs-0445 */                         fn visit_infer(
/* FP:resolve_bound_vars.rs-0446 */                             &mut self,
/* FP:resolve_bound_vars.rs-0447 */                             _inf_id: HirId,
/* FP:resolve_bound_vars.rs-0448 */                             inf_span: Span,
/* FP:resolve_bound_vars.rs-0449 */                             _kind: InferKind<'v>,
/* FP:resolve_bound_vars.rs-0450 */                         ) -> Self::Result {
/* FP:resolve_bound_vars.rs-0451 */                             ControlFlow::Break(inf_span)
/* FP:resolve_bound_vars.rs-0452 */                         }
/* FP:resolve_bound_vars.rs-0453 */                     }
/* FP:resolve_bound_vars.rs-0454 */                     FindInferInClosureWithBinder.visit_ty_unambig(ty).break_value()
/* FP:resolve_bound_vars.rs-0455 */                 }
/* FP:resolve_bound_vars.rs-0456 */ 
/* FP:resolve_bound_vars.rs-0457 */                 let infer_in_rt_sp = match fn_decl.output {
/* FP:resolve_bound_vars.rs-0458 */                     hir::FnRetTy::DefaultReturn(sp) => Some(sp),
/* FP:resolve_bound_vars.rs-0459 */                     hir::FnRetTy::Return(ty) => span_of_infer(ty),
/* FP:resolve_bound_vars.rs-0460 */                 };
/* FP:resolve_bound_vars.rs-0461 */ 
/* FP:resolve_bound_vars.rs-0462 */                 let infer_spans = fn_decl
/* FP:resolve_bound_vars.rs-0463 */                     .inputs
/* FP:resolve_bound_vars.rs-0464 */                     .into_iter()
/* FP:resolve_bound_vars.rs-0465 */                     .filter_map(span_of_infer)
/* FP:resolve_bound_vars.rs-0466 */                     .chain(infer_in_rt_sp)
/* FP:resolve_bound_vars.rs-0467 */                     .collect::<Vec<_>>();
/* FP:resolve_bound_vars.rs-0468 */ 
/* FP:resolve_bound_vars.rs-0469 */                 if !infer_spans.is_empty() {
/* FP:resolve_bound_vars.rs-0470 */                     self.tcx
/* FP:resolve_bound_vars.rs-0471 */                         .dcx()
/* FP:resolve_bound_vars.rs-0472 */                         .emit_err(errors::ClosureImplicitHrtb { spans: infer_spans, for_sp });
/* FP:resolve_bound_vars.rs-0473 */                 }
/* FP:resolve_bound_vars.rs-0474 */             }
/* FP:resolve_bound_vars.rs-0475 */ 
/* FP:resolve_bound_vars.rs-0476 */             let (mut bound_vars, binders): (FxIndexMap<LocalDefId, ResolvedArg>, Vec<_>) =
/* FP:resolve_bound_vars.rs-0477 */                 bound_generic_params
/* FP:resolve_bound_vars.rs-0478 */                     .iter()
/* FP:resolve_bound_vars.rs-0479 */                     .enumerate()
/* FP:resolve_bound_vars.rs-0480 */                     .map(|(late_bound_idx, param)| {
/* FP:resolve_bound_vars.rs-0481 */                         (
/* FP:resolve_bound_vars.rs-0482 */                             (param.def_id, ResolvedArg::late(late_bound_idx as u32, param)),
/* FP:resolve_bound_vars.rs-0483 */                             late_arg_as_bound_arg(param),
/* FP:resolve_bound_vars.rs-0484 */                         )
/* FP:resolve_bound_vars.rs-0485 */                     })
/* FP:resolve_bound_vars.rs-0486 */                     .unzip();
/* FP:resolve_bound_vars.rs-0487 */ 
/* FP:resolve_bound_vars.rs-0488 */             deny_non_region_late_bound(self.tcx, &mut bound_vars, "closures");
/* FP:resolve_bound_vars.rs-0489 */ 
/* FP:resolve_bound_vars.rs-0490 */             self.record_late_bound_vars(e.hir_id, binders);
/* FP:resolve_bound_vars.rs-0491 */             let scope = Scope::Binder {
/* FP:resolve_bound_vars.rs-0492 */                 hir_id: e.hir_id,
/* FP:resolve_bound_vars.rs-0493 */                 bound_vars,
/* FP:resolve_bound_vars.rs-0494 */                 s: self.scope,
/* FP:resolve_bound_vars.rs-0495 */                 scope_type: BinderScopeType::Normal,
/* FP:resolve_bound_vars.rs-0496 */                 where_bound_origin: None,
/* FP:resolve_bound_vars.rs-0497 */             };
/* FP:resolve_bound_vars.rs-0498 */ 
/* FP:resolve_bound_vars.rs-0499 */             self.with(scope, |this| {
/* FP:resolve_bound_vars.rs-0500 */                 // a closure has no bounds, so everything
/* FP:resolve_bound_vars.rs-0501 */                 // contained within is scoped within its binder.
/* FP:resolve_bound_vars.rs-0502 */                 intravisit::walk_expr(this, e)
/* FP:resolve_bound_vars.rs-0503 */             });
/* FP:resolve_bound_vars.rs-0504 */         } else {
/* FP:resolve_bound_vars.rs-0505 */             intravisit::walk_expr(self, e)
/* FP:resolve_bound_vars.rs-0506 */         }
/* FP:resolve_bound_vars.rs-0507 */     }
/* FP:resolve_bound_vars.rs-0508 */ 
/* FP:resolve_bound_vars.rs-0509 */     /// Resolve the lifetimes inside the opaque type, and save them into
/* FP:resolve_bound_vars.rs-0510 */     /// `opaque_captured_lifetimes`.
/* FP:resolve_bound_vars.rs-0511 */     ///
/* FP:resolve_bound_vars.rs-0512 */     /// This method has special handling for opaques that capture all lifetimes,
/* FP:resolve_bound_vars.rs-0513 */     /// like async desugaring.
/* FP:resolve_bound_vars.rs-0514 */     #[instrument(level = "debug", skip(self))]
/* FP:resolve_bound_vars.rs-0515 */     fn visit_opaque_ty(&mut self, opaque: &'tcx crate::rustc_hir::OpaqueTy<'tcx>) {
/* FP:resolve_bound_vars.rs-0516 */         let captures = RefCell::new(FxIndexMap::default());
/* FP:resolve_bound_vars.rs-0517 */ 
/* FP:resolve_bound_vars.rs-0518 */         let capture_all_in_scope_lifetimes = opaque_captures_all_in_scope_lifetimes(opaque);
/* FP:resolve_bound_vars.rs-0519 */         if capture_all_in_scope_lifetimes {
/* FP:resolve_bound_vars.rs-0520 */             let tcx = self.tcx;
/* FP:resolve_bound_vars.rs-0521 */             let lifetime_ident = |def_id: LocalDefId| {
/* FP:resolve_bound_vars.rs-0522 */                 let name = tcx.item_name(def_id.to_def_id());
/* FP:resolve_bound_vars.rs-0523 */                 let span = tcx.def_span(def_id);
/* FP:resolve_bound_vars.rs-0524 */                 Ident::new(name, span)
/* FP:resolve_bound_vars.rs-0525 */             };
/* FP:resolve_bound_vars.rs-0526 */ 
/* FP:resolve_bound_vars.rs-0527 */             // We list scopes outwards, this causes us to see lifetime parameters in reverse
/* FP:resolve_bound_vars.rs-0528 */             // declaration order. In order to make it consistent with what `generics_of` might
/* FP:resolve_bound_vars.rs-0529 */             // give, we will reverse the IndexMap after early captures.
/* FP:resolve_bound_vars.rs-0530 */             let mut late_depth = 0;
/* FP:resolve_bound_vars.rs-0531 */             let mut scope = self.scope;
/* FP:resolve_bound_vars.rs-0532 */             let mut opaque_capture_scopes = vec![(opaque.def_id, &captures)];
/* FP:resolve_bound_vars.rs-0533 */             loop {
/* FP:resolve_bound_vars.rs-0534 */                 match *scope {
/* FP:resolve_bound_vars.rs-0535 */                     Scope::Binder { ref bound_vars, scope_type, s, .. } => {
/* FP:resolve_bound_vars.rs-0536 */                         for (&original_lifetime, &def) in bound_vars.iter().rev() {
/* FP:resolve_bound_vars.rs-0537 */                             if let DefKind::LifetimeParam = self.tcx.def_kind(original_lifetime) {
/* FP:resolve_bound_vars.rs-0538 */                                 let def = def.shifted(late_depth);
/* FP:resolve_bound_vars.rs-0539 */                                 let ident = lifetime_ident(original_lifetime);
/* FP:resolve_bound_vars.rs-0540 */                                 self.remap_opaque_captures(&opaque_capture_scopes, def, ident);
/* FP:resolve_bound_vars.rs-0541 */                             }
/* FP:resolve_bound_vars.rs-0542 */                         }
/* FP:resolve_bound_vars.rs-0543 */                         match scope_type {
/* FP:resolve_bound_vars.rs-0544 */                             BinderScopeType::Normal => late_depth += 1,
/* FP:resolve_bound_vars.rs-0545 */                             BinderScopeType::Concatenating => {}
/* FP:resolve_bound_vars.rs-0546 */                         }
/* FP:resolve_bound_vars.rs-0547 */                         scope = s;
/* FP:resolve_bound_vars.rs-0548 */                     }
/* FP:resolve_bound_vars.rs-0549 */ 
/* FP:resolve_bound_vars.rs-0550 */                     Scope::Root { mut opt_parent_item } => {
/* FP:resolve_bound_vars.rs-0551 */                         while let Some(parent_item) = opt_parent_item {
/* FP:resolve_bound_vars.rs-0552 */                             let parent_generics = self.tcx.generics_of(parent_item);
/* FP:resolve_bound_vars.rs-0553 */                             for param in parent_generics.own_params.iter().rev() {
/* FP:resolve_bound_vars.rs-0554 */                                 if let ty::GenericParamDefKind::Lifetime = param.kind {
/* FP:resolve_bound_vars.rs-0555 */                                     let def = ResolvedArg::EarlyBound(param.def_id.expect_local());
/* FP:resolve_bound_vars.rs-0556 */                                     let ident = lifetime_ident(param.def_id.expect_local());
/* FP:resolve_bound_vars.rs-0557 */                                     self.remap_opaque_captures(&opaque_capture_scopes, def, ident);
/* FP:resolve_bound_vars.rs-0558 */                                 }
/* FP:resolve_bound_vars.rs-0559 */                             }
/* FP:resolve_bound_vars.rs-0560 */                             opt_parent_item = parent_generics.parent.and_then(DefId::as_local);
/* FP:resolve_bound_vars.rs-0561 */                         }
/* FP:resolve_bound_vars.rs-0562 */                         break;
/* FP:resolve_bound_vars.rs-0563 */                     }
/* FP:resolve_bound_vars.rs-0564 */ 
/* FP:resolve_bound_vars.rs-0565 */                     Scope::Opaque { captures, def_id, s } => {
/* FP:resolve_bound_vars.rs-0566 */                         opaque_capture_scopes.push((def_id, captures));
/* FP:resolve_bound_vars.rs-0567 */                         late_depth = 0;
/* FP:resolve_bound_vars.rs-0568 */                         scope = s;
/* FP:resolve_bound_vars.rs-0569 */                     }
/* FP:resolve_bound_vars.rs-0570 */ 
/* FP:resolve_bound_vars.rs-0571 */                     Scope::Body { .. } => {
/* FP:resolve_bound_vars.rs-0572 */                         bug!("{:?}", scope)
/* FP:resolve_bound_vars.rs-0573 */                     }
/* FP:resolve_bound_vars.rs-0574 */ 
/* FP:resolve_bound_vars.rs-0575 */                     Scope::ObjectLifetimeDefault { s, .. }
/* FP:resolve_bound_vars.rs-0576 */                     | Scope::Supertrait { s, .. }
/* FP:resolve_bound_vars.rs-0577 */                     | Scope::TraitRefBoundary { s, .. }
/* FP:resolve_bound_vars.rs-0578 */                     | Scope::LateBoundary { s, .. } => {
/* FP:resolve_bound_vars.rs-0579 */                         scope = s;
/* FP:resolve_bound_vars.rs-0580 */                     }
/* FP:resolve_bound_vars.rs-0581 */                 }
/* FP:resolve_bound_vars.rs-0582 */             }
/* FP:resolve_bound_vars.rs-0583 */             captures.borrow_mut().reverse();
/* FP:resolve_bound_vars.rs-0584 */         }
/* FP:resolve_bound_vars.rs-0585 */ 
/* FP:resolve_bound_vars.rs-0586 */         let scope = Scope::Opaque { captures: &captures, def_id: opaque.def_id, s: self.scope };
/* FP:resolve_bound_vars.rs-0587 */         self.with(scope, |this| {
/* FP:resolve_bound_vars.rs-0588 */             let scope = Scope::TraitRefBoundary { s: this.scope };
/* FP:resolve_bound_vars.rs-0589 */             this.with(scope, |this| {
/* FP:resolve_bound_vars.rs-0590 */                 let scope = Scope::LateBoundary {
/* FP:resolve_bound_vars.rs-0591 */                     s: this.scope,
/* FP:resolve_bound_vars.rs-0592 */                     what: "nested `impl Trait`",
/* FP:resolve_bound_vars.rs-0593 */                     // We can capture late-bound regions; we just don't duplicate
/* FP:resolve_bound_vars.rs-0594 */                     // lifetime or const params, so we can't allow those.
/* FP:resolve_bound_vars.rs-0595 */                     deny_late_regions: false,
/* FP:resolve_bound_vars.rs-0596 */                 };
/* FP:resolve_bound_vars.rs-0597 */                 this.with(scope, |this| intravisit::walk_opaque_ty(this, opaque))
/* FP:resolve_bound_vars.rs-0598 */             })
/* FP:resolve_bound_vars.rs-0599 */         });
/* FP:resolve_bound_vars.rs-0600 */ 
/* FP:resolve_bound_vars.rs-0601 */         let captures = captures.into_inner().into_iter().collect();
/* FP:resolve_bound_vars.rs-0602 */         debug!(?captures);
/* FP:resolve_bound_vars.rs-0603 */         self.rbv.opaque_captured_lifetimes.insert(opaque.def_id, captures);
/* FP:resolve_bound_vars.rs-0604 */     }
/* FP:resolve_bound_vars.rs-0605 */ 
/* FP:resolve_bound_vars.rs-0606 */     #[instrument(level = "debug", skip(self))]
/* FP:resolve_bound_vars.rs-0607 */     fn visit_item(&mut self, item: &'tcx hir::Item<'tcx>) {
/* FP:resolve_bound_vars.rs-0608 */         if let hir::ItemKind::Impl(impl_) = item.kind
/* FP:resolve_bound_vars.rs-0609 */             && let Some(of_trait) = impl_.of_trait
/* FP:resolve_bound_vars.rs-0610 */         {
/* FP:resolve_bound_vars.rs-0611 */             self.record_late_bound_vars(of_trait.trait_ref.hir_ref_id, Vec::default());
/* FP:resolve_bound_vars.rs-0612 */         }
/* FP:resolve_bound_vars.rs-0613 */         match item.kind {
/* FP:resolve_bound_vars.rs-0614 */             hir::ItemKind::Fn { generics, .. } => {
/* FP:resolve_bound_vars.rs-0615 */                 self.visit_early_late(item.hir_id(), generics, |this| {
/* FP:resolve_bound_vars.rs-0616 */                     intravisit::walk_item(this, item);
/* FP:resolve_bound_vars.rs-0617 */                 });
/* FP:resolve_bound_vars.rs-0618 */             }
/* FP:resolve_bound_vars.rs-0619 */ 
/* FP:resolve_bound_vars.rs-0620 */             hir::ItemKind::ExternCrate(..)
/* FP:resolve_bound_vars.rs-0621 */             | hir::ItemKind::Use(..)
/* FP:resolve_bound_vars.rs-0622 */             | hir::ItemKind::Macro(..)
/* FP:resolve_bound_vars.rs-0623 */             | hir::ItemKind::Mod(..)
/* FP:resolve_bound_vars.rs-0624 */             | hir::ItemKind::ForeignMod { .. }
/* FP:resolve_bound_vars.rs-0625 */             | hir::ItemKind::Static(..)
/* FP:resolve_bound_vars.rs-0626 */             | hir::ItemKind::GlobalAsm { .. } => {
/* FP:resolve_bound_vars.rs-0627 */                 // These sorts of items have no lifetime parameters at all.
/* FP:resolve_bound_vars.rs-0628 */                 intravisit::walk_item(self, item);
/* FP:resolve_bound_vars.rs-0629 */             }
/* FP:resolve_bound_vars.rs-0630 */             hir::ItemKind::TyAlias(_, generics, _)
/* FP:resolve_bound_vars.rs-0631 */             | hir::ItemKind::Const(_, generics, _, _)
/* FP:resolve_bound_vars.rs-0632 */             | hir::ItemKind::Enum(_, generics, _)
/* FP:resolve_bound_vars.rs-0633 */             | hir::ItemKind::Struct(_, generics, _)
/* FP:resolve_bound_vars.rs-0634 */             | hir::ItemKind::Union(_, generics, _)
/* FP:resolve_bound_vars.rs-0635 */             | hir::ItemKind::Trait(_, _, _, _, generics, ..)
/* FP:resolve_bound_vars.rs-0636 */             | hir::ItemKind::TraitAlias(_, generics, ..)
/* FP:resolve_bound_vars.rs-0637 */             | hir::ItemKind::Impl(hir::Impl { generics, .. }) => {
/* FP:resolve_bound_vars.rs-0638 */                 // These kinds of items have only early-bound lifetime parameters.
/* FP:resolve_bound_vars.rs-0639 */                 self.visit_early(item.hir_id(), generics, |this| intravisit::walk_item(this, item));
/* FP:resolve_bound_vars.rs-0640 */             }
/* FP:resolve_bound_vars.rs-0641 */         }
/* FP:resolve_bound_vars.rs-0642 */     }
/* FP:resolve_bound_vars.rs-0643 */ 
/* FP:resolve_bound_vars.rs-0644 */     fn visit_precise_capturing_arg(
/* FP:resolve_bound_vars.rs-0645 */         &mut self,
/* FP:resolve_bound_vars.rs-0646 */         arg: &'tcx hir::PreciseCapturingArg<'tcx>,
/* FP:resolve_bound_vars.rs-0647 */     ) -> Self::Result {
/* FP:resolve_bound_vars.rs-0648 */         match *arg {
/* FP:resolve_bound_vars.rs-0649 */             hir::PreciseCapturingArg::Lifetime(lt) => match lt.kind {
/* FP:resolve_bound_vars.rs-0650 */                 LifetimeKind::Param(def_id) => {
/* FP:resolve_bound_vars.rs-0651 */                     self.resolve_lifetime_ref(def_id, lt);
/* FP:resolve_bound_vars.rs-0652 */                 }
/* FP:resolve_bound_vars.rs-0653 */                 LifetimeKind::Error => {}
/* FP:resolve_bound_vars.rs-0654 */                 LifetimeKind::ImplicitObjectLifetimeDefault
/* FP:resolve_bound_vars.rs-0655 */                 | LifetimeKind::Infer
/* FP:resolve_bound_vars.rs-0656 */                 | LifetimeKind::Static => {
/* FP:resolve_bound_vars.rs-0657 */                     self.tcx.dcx().emit_err(errors::BadPreciseCapture {
/* FP:resolve_bound_vars.rs-0658 */                         span: lt.ident.span,
/* FP:resolve_bound_vars.rs-0659 */                         kind: "lifetime",
/* FP:resolve_bound_vars.rs-0660 */                         found: format!("`{}`", lt.ident.name),
/* FP:resolve_bound_vars.rs-0661 */                     });
/* FP:resolve_bound_vars.rs-0662 */                 }
/* FP:resolve_bound_vars.rs-0663 */             },
/* FP:resolve_bound_vars.rs-0664 */             hir::PreciseCapturingArg::Param(param) => match param.res {
/* FP:resolve_bound_vars.rs-0665 */                 Res::Def(DefKind::TyParam | DefKind::ConstParam, def_id)
/* FP:resolve_bound_vars.rs-0666 */                 | Res::SelfTyParam { trait_: def_id } => {
/* FP:resolve_bound_vars.rs-0667 */                     self.resolve_type_ref(def_id.expect_local(), param.hir_id);
/* FP:resolve_bound_vars.rs-0668 */                 }
/* FP:resolve_bound_vars.rs-0669 */                 Res::SelfTyAlias { alias_to, .. } => {
/* FP:resolve_bound_vars.rs-0670 */                     self.tcx.dcx().emit_err(errors::PreciseCaptureSelfAlias {
/* FP:resolve_bound_vars.rs-0671 */                         span: param.ident.span,
/* FP:resolve_bound_vars.rs-0672 */                         self_span: self.tcx.def_span(alias_to),
/* FP:resolve_bound_vars.rs-0673 */                         what: self.tcx.def_descr(alias_to),
/* FP:resolve_bound_vars.rs-0674 */                     });
/* FP:resolve_bound_vars.rs-0675 */                 }
/* FP:resolve_bound_vars.rs-0676 */                 res => {
/* FP:resolve_bound_vars.rs-0677 */                     self.tcx.dcx().span_delayed_bug(
/* FP:resolve_bound_vars.rs-0678 */                         param.ident.span,
/* FP:resolve_bound_vars.rs-0679 */                         format!("expected type or const param, found {res:?}"),
/* FP:resolve_bound_vars.rs-0680 */                     );
/* FP:resolve_bound_vars.rs-0681 */                 }
/* FP:resolve_bound_vars.rs-0682 */             },
/* FP:resolve_bound_vars.rs-0683 */         }
/* FP:resolve_bound_vars.rs-0684 */     }
/* FP:resolve_bound_vars.rs-0685 */ 
/* FP:resolve_bound_vars.rs-0686 */     fn visit_foreign_item(&mut self, item: &'tcx hir::ForeignItem<'tcx>) {
/* FP:resolve_bound_vars.rs-0687 */         match item.kind {
/* FP:resolve_bound_vars.rs-0688 */             hir::ForeignItemKind::Fn(_, _, generics) => {
/* FP:resolve_bound_vars.rs-0689 */                 self.visit_early_late(item.hir_id(), generics, |this| {
/* FP:resolve_bound_vars.rs-0690 */                     intravisit::walk_foreign_item(this, item);
/* FP:resolve_bound_vars.rs-0691 */                 })
/* FP:resolve_bound_vars.rs-0692 */             }
/* FP:resolve_bound_vars.rs-0693 */             hir::ForeignItemKind::Static(..) => {
/* FP:resolve_bound_vars.rs-0694 */                 intravisit::walk_foreign_item(self, item);
/* FP:resolve_bound_vars.rs-0695 */             }
/* FP:resolve_bound_vars.rs-0696 */             hir::ForeignItemKind::Type => {
/* FP:resolve_bound_vars.rs-0697 */                 intravisit::walk_foreign_item(self, item);
/* FP:resolve_bound_vars.rs-0698 */             }
/* FP:resolve_bound_vars.rs-0699 */         }
/* FP:resolve_bound_vars.rs-0700 */     }
/* FP:resolve_bound_vars.rs-0701 */ 
/* FP:resolve_bound_vars.rs-0702 */     #[instrument(level = "debug", skip(self))]
/* FP:resolve_bound_vars.rs-0703 */     fn visit_ty(&mut self, ty: &'tcx hir::Ty<'tcx, AmbigArg>) {
/* FP:resolve_bound_vars.rs-0704 */         match ty.kind {
/* FP:resolve_bound_vars.rs-0705 */             hir::TyKind::FnPtr(c) => {
/* FP:resolve_bound_vars.rs-0706 */                 let (mut bound_vars, binders): (FxIndexMap<LocalDefId, ResolvedArg>, Vec<_>) = c
/* FP:resolve_bound_vars.rs-0707 */                     .generic_params
/* FP:resolve_bound_vars.rs-0708 */                     .iter()
/* FP:resolve_bound_vars.rs-0709 */                     .enumerate()
/* FP:resolve_bound_vars.rs-0710 */                     .map(|(late_bound_idx, param)| {
/* FP:resolve_bound_vars.rs-0711 */                         (
/* FP:resolve_bound_vars.rs-0712 */                             (param.def_id, ResolvedArg::late(late_bound_idx as u32, param)),
/* FP:resolve_bound_vars.rs-0713 */                             late_arg_as_bound_arg(param),
/* FP:resolve_bound_vars.rs-0714 */                         )
/* FP:resolve_bound_vars.rs-0715 */                     })
/* FP:resolve_bound_vars.rs-0716 */                     .unzip();
/* FP:resolve_bound_vars.rs-0717 */ 
/* FP:resolve_bound_vars.rs-0718 */                 deny_non_region_late_bound(self.tcx, &mut bound_vars, "function pointer types");
/* FP:resolve_bound_vars.rs-0719 */ 
/* FP:resolve_bound_vars.rs-0720 */                 self.record_late_bound_vars(ty.hir_id, binders);
/* FP:resolve_bound_vars.rs-0721 */                 let scope = Scope::Binder {
/* FP:resolve_bound_vars.rs-0722 */                     hir_id: ty.hir_id,
/* FP:resolve_bound_vars.rs-0723 */                     bound_vars,
/* FP:resolve_bound_vars.rs-0724 */                     s: self.scope,
/* FP:resolve_bound_vars.rs-0725 */                     scope_type: BinderScopeType::Normal,
/* FP:resolve_bound_vars.rs-0726 */                     where_bound_origin: None,
/* FP:resolve_bound_vars.rs-0727 */                 };
/* FP:resolve_bound_vars.rs-0728 */                 self.with(scope, |this| {
/* FP:resolve_bound_vars.rs-0729 */                     // a FnPtr has no bounds, so everything within is scoped within its binder
/* FP:resolve_bound_vars.rs-0730 */                     intravisit::walk_ty(this, ty);
/* FP:resolve_bound_vars.rs-0731 */                 });
/* FP:resolve_bound_vars.rs-0732 */             }
/* FP:resolve_bound_vars.rs-0733 */             hir::TyKind::UnsafeBinder(binder) => {
/* FP:resolve_bound_vars.rs-0734 */                 let (mut bound_vars, binders): (FxIndexMap<LocalDefId, ResolvedArg>, Vec<_>) =
/* FP:resolve_bound_vars.rs-0735 */                     binder
/* FP:resolve_bound_vars.rs-0736 */                         .generic_params
/* FP:resolve_bound_vars.rs-0737 */                         .iter()
/* FP:resolve_bound_vars.rs-0738 */                         .enumerate()
/* FP:resolve_bound_vars.rs-0739 */                         .map(|(late_bound_idx, param)| {
/* FP:resolve_bound_vars.rs-0740 */                             (
/* FP:resolve_bound_vars.rs-0741 */                                 (param.def_id, ResolvedArg::late(late_bound_idx as u32, param)),
/* FP:resolve_bound_vars.rs-0742 */                                 late_arg_as_bound_arg(param),
/* FP:resolve_bound_vars.rs-0743 */                             )
/* FP:resolve_bound_vars.rs-0744 */                         })
/* FP:resolve_bound_vars.rs-0745 */                         .unzip();
/* FP:resolve_bound_vars.rs-0746 */ 
/* FP:resolve_bound_vars.rs-0747 */                 deny_non_region_late_bound(self.tcx, &mut bound_vars, "function pointer types");
/* FP:resolve_bound_vars.rs-0748 */ 
/* FP:resolve_bound_vars.rs-0749 */                 self.record_late_bound_vars(ty.hir_id, binders);
/* FP:resolve_bound_vars.rs-0750 */                 let scope = Scope::Binder {
/* FP:resolve_bound_vars.rs-0751 */                     hir_id: ty.hir_id,
/* FP:resolve_bound_vars.rs-0752 */                     bound_vars,
/* FP:resolve_bound_vars.rs-0753 */                     s: self.scope,
/* FP:resolve_bound_vars.rs-0754 */                     scope_type: BinderScopeType::Normal,
/* FP:resolve_bound_vars.rs-0755 */                     where_bound_origin: None,
/* FP:resolve_bound_vars.rs-0756 */                 };
/* FP:resolve_bound_vars.rs-0757 */                 self.with(scope, |this| {
/* FP:resolve_bound_vars.rs-0758 */                     // everything within is scoped within its binder
/* FP:resolve_bound_vars.rs-0759 */                     intravisit::walk_ty(this, ty);
/* FP:resolve_bound_vars.rs-0760 */                 });
/* FP:resolve_bound_vars.rs-0761 */             }
/* FP:resolve_bound_vars.rs-0762 */             hir::TyKind::TraitObject(bounds, lifetime) => {
/* FP:resolve_bound_vars.rs-0763 */                 let lifetime = lifetime.pointer();
/* FP:resolve_bound_vars.rs-0764 */ 
/* FP:resolve_bound_vars.rs-0765 */                 debug!(?bounds, ?lifetime, "TraitObject");
/* FP:resolve_bound_vars.rs-0766 */                 let scope = Scope::TraitRefBoundary { s: self.scope };
/* FP:resolve_bound_vars.rs-0767 */                 self.with(scope, |this| {
/* FP:resolve_bound_vars.rs-0768 */                     for bound in bounds {
/* FP:resolve_bound_vars.rs-0769 */                         this.visit_poly_trait_ref_inner(
/* FP:resolve_bound_vars.rs-0770 */                             bound,
/* FP:resolve_bound_vars.rs-0771 */                             NonLifetimeBinderAllowed::Deny("trait object types"),
/* FP:resolve_bound_vars.rs-0772 */                         );
/* FP:resolve_bound_vars.rs-0773 */                     }
/* FP:resolve_bound_vars.rs-0774 */                 });
/* FP:resolve_bound_vars.rs-0775 */                 match lifetime.kind {
/* FP:resolve_bound_vars.rs-0776 */                     LifetimeKind::ImplicitObjectLifetimeDefault => {
/* FP:resolve_bound_vars.rs-0777 */                         // If the user does not write *anything*, we
/* FP:resolve_bound_vars.rs-0778 */                         // use the object lifetime defaulting
/* FP:resolve_bound_vars.rs-0779 */                         // rules. So e.g., `Box<dyn Debug>` becomes
/* FP:resolve_bound_vars.rs-0780 */                         // `Box<dyn Debug + 'static>`.
/* FP:resolve_bound_vars.rs-0781 */                         self.resolve_object_lifetime_default(&*lifetime)
/* FP:resolve_bound_vars.rs-0782 */                     }
/* FP:resolve_bound_vars.rs-0783 */                     LifetimeKind::Infer => {
/* FP:resolve_bound_vars.rs-0784 */                         // If the user writes `'_`, we use the *ordinary* elision
/* FP:resolve_bound_vars.rs-0785 */                         // rules. So the `'_` in e.g., `Box<dyn Debug + '_>` will be
/* FP:resolve_bound_vars.rs-0786 */                         // resolved the same as the `'_` in `&'_ Foo`.
/* FP:resolve_bound_vars.rs-0787 */                         //
/* FP:resolve_bound_vars.rs-0788 */                         // cc #48468
/* FP:resolve_bound_vars.rs-0789 */                     }
/* FP:resolve_bound_vars.rs-0790 */                     LifetimeKind::Param(..) | LifetimeKind::Static => {
/* FP:resolve_bound_vars.rs-0791 */                         // If the user wrote an explicit name, use that.
/* FP:resolve_bound_vars.rs-0792 */                         self.visit_lifetime(&*lifetime);
/* FP:resolve_bound_vars.rs-0793 */                     }
/* FP:resolve_bound_vars.rs-0794 */                     LifetimeKind::Error => {}
/* FP:resolve_bound_vars.rs-0795 */                 }
/* FP:resolve_bound_vars.rs-0796 */             }
/* FP:resolve_bound_vars.rs-0797 */             hir::TyKind::Ref(lifetime_ref, ref mt) => {
/* FP:resolve_bound_vars.rs-0798 */                 self.visit_lifetime(lifetime_ref);
/* FP:resolve_bound_vars.rs-0799 */                 let scope = Scope::ObjectLifetimeDefault {
/* FP:resolve_bound_vars.rs-0800 */                     lifetime: self.rbv.defs.get(&lifetime_ref.hir_id.local_id).cloned(),
/* FP:resolve_bound_vars.rs-0801 */                     s: self.scope,
/* FP:resolve_bound_vars.rs-0802 */                 };
/* FP:resolve_bound_vars.rs-0803 */                 self.with(scope, |this| this.visit_ty_unambig(mt.ty));
/* FP:resolve_bound_vars.rs-0804 */             }
/* FP:resolve_bound_vars.rs-0805 */             hir::TyKind::TraitAscription(bounds) => {
/* FP:resolve_bound_vars.rs-0806 */                 let scope = Scope::TraitRefBoundary { s: self.scope };
/* FP:resolve_bound_vars.rs-0807 */                 self.with(scope, |this| {
/* FP:resolve_bound_vars.rs-0808 */                     let scope = Scope::LateBoundary {
/* FP:resolve_bound_vars.rs-0809 */                         s: this.scope,
/* FP:resolve_bound_vars.rs-0810 */                         what: "`impl Trait` in binding",
/* FP:resolve_bound_vars.rs-0811 */                         deny_late_regions: true,
/* FP:resolve_bound_vars.rs-0812 */                     };
/* FP:resolve_bound_vars.rs-0813 */                     this.with(scope, |this| {
/* FP:resolve_bound_vars.rs-0814 */                         for bound in bounds {
/* FP:resolve_bound_vars.rs-0815 */                             this.visit_param_bound(bound);
/* FP:resolve_bound_vars.rs-0816 */                         }
/* FP:resolve_bound_vars.rs-0817 */                     })
/* FP:resolve_bound_vars.rs-0818 */                 });
/* FP:resolve_bound_vars.rs-0819 */             }
/* FP:resolve_bound_vars.rs-0820 */             _ => intravisit::walk_ty(self, ty),
/* FP:resolve_bound_vars.rs-0821 */         }
/* FP:resolve_bound_vars.rs-0822 */     }
/* FP:resolve_bound_vars.rs-0823 */ 
/* FP:resolve_bound_vars.rs-0824 */     #[instrument(level = "debug", skip(self))]
/* FP:resolve_bound_vars.rs-0825 */     fn visit_pattern_type_pattern(&mut self, p: &'tcx hir::TyPat<'tcx>) {
/* FP:resolve_bound_vars.rs-0826 */         intravisit::walk_ty_pat(self, p)
/* FP:resolve_bound_vars.rs-0827 */     }
/* FP:resolve_bound_vars.rs-0828 */ 
/* FP:resolve_bound_vars.rs-0829 */     #[instrument(level = "debug", skip(self))]
/* FP:resolve_bound_vars.rs-0830 */     fn visit_trait_item(&mut self, trait_item: &'tcx hir::TraitItem<'tcx>) {
/* FP:resolve_bound_vars.rs-0831 */         use self::hir::TraitItemKind::*;
/* FP:resolve_bound_vars.rs-0832 */         match trait_item.kind {
/* FP:resolve_bound_vars.rs-0833 */             Fn(_, _) => {
/* FP:resolve_bound_vars.rs-0834 */                 self.visit_early_late(trait_item.hir_id(), trait_item.generics, |this| {
/* FP:resolve_bound_vars.rs-0835 */                     intravisit::walk_trait_item(this, trait_item)
/* FP:resolve_bound_vars.rs-0836 */                 });
/* FP:resolve_bound_vars.rs-0837 */             }
/* FP:resolve_bound_vars.rs-0838 */             Type(bounds, ty) => {
/* FP:resolve_bound_vars.rs-0839 */                 self.visit_early(trait_item.hir_id(), trait_item.generics, |this| {
/* FP:resolve_bound_vars.rs-0840 */                     this.visit_generics(trait_item.generics);
/* FP:resolve_bound_vars.rs-0841 */                     for bound in bounds {
/* FP:resolve_bound_vars.rs-0842 */                         this.visit_param_bound(bound);
/* FP:resolve_bound_vars.rs-0843 */                     }
/* FP:resolve_bound_vars.rs-0844 */                     if let Some(ty) = ty {
/* FP:resolve_bound_vars.rs-0845 */                         this.visit_ty_unambig(ty);
/* FP:resolve_bound_vars.rs-0846 */                     }
/* FP:resolve_bound_vars.rs-0847 */                 })
/* FP:resolve_bound_vars.rs-0848 */             }
/* FP:resolve_bound_vars.rs-0849 */             Const(_, _) => self.visit_early(trait_item.hir_id(), trait_item.generics, |this| {
/* FP:resolve_bound_vars.rs-0850 */                 intravisit::walk_trait_item(this, trait_item)
/* FP:resolve_bound_vars.rs-0851 */             }),
/* FP:resolve_bound_vars.rs-0852 */         }
/* FP:resolve_bound_vars.rs-0853 */     }
/* FP:resolve_bound_vars.rs-0854 */ 
/* FP:resolve_bound_vars.rs-0855 */     #[instrument(level = "debug", skip(self))]
/* FP:resolve_bound_vars.rs-0856 */     fn visit_impl_item(&mut self, impl_item: &'tcx hir::ImplItem<'tcx>) {
/* FP:resolve_bound_vars.rs-0857 */         use self::hir::ImplItemKind::*;
/* FP:resolve_bound_vars.rs-0858 */         match impl_item.kind {
/* FP:resolve_bound_vars.rs-0859 */             Fn(..) => self.visit_early_late(impl_item.hir_id(), impl_item.generics, |this| {
/* FP:resolve_bound_vars.rs-0860 */                 intravisit::walk_impl_item(this, impl_item)
/* FP:resolve_bound_vars.rs-0861 */             }),
/* FP:resolve_bound_vars.rs-0862 */             Type(ty) => self.visit_early(impl_item.hir_id(), impl_item.generics, |this| {
/* FP:resolve_bound_vars.rs-0863 */                 this.visit_generics(impl_item.generics);
/* FP:resolve_bound_vars.rs-0864 */                 this.visit_ty_unambig(ty);
/* FP:resolve_bound_vars.rs-0865 */             }),
/* FP:resolve_bound_vars.rs-0866 */             Const(_, _) => self.visit_early(impl_item.hir_id(), impl_item.generics, |this| {
/* FP:resolve_bound_vars.rs-0867 */                 intravisit::walk_impl_item(this, impl_item)
/* FP:resolve_bound_vars.rs-0868 */             }),
/* FP:resolve_bound_vars.rs-0869 */         }
/* FP:resolve_bound_vars.rs-0870 */     }
/* FP:resolve_bound_vars.rs-0871 */ 
/* FP:resolve_bound_vars.rs-0872 */     #[instrument(level = "debug", skip(self))]
/* FP:resolve_bound_vars.rs-0873 */     fn visit_lifetime(&mut self, lifetime_ref: &'tcx hir::Lifetime) {
/* FP:resolve_bound_vars.rs-0874 */         match lifetime_ref.kind {
/* FP:resolve_bound_vars.rs-0875 */             hir::LifetimeKind::Static => {
/* FP:resolve_bound_vars.rs-0876 */                 self.insert_lifetime(lifetime_ref, ResolvedArg::StaticLifetime)
/* FP:resolve_bound_vars.rs-0877 */             }
/* FP:resolve_bound_vars.rs-0878 */             hir::LifetimeKind::Param(param_def_id) => {
/* FP:resolve_bound_vars.rs-0879 */                 self.resolve_lifetime_ref(param_def_id, lifetime_ref)
/* FP:resolve_bound_vars.rs-0880 */             }
/* FP:resolve_bound_vars.rs-0881 */             // If we've already reported an error, just ignore `lifetime_ref`.
/* FP:resolve_bound_vars.rs-0882 */             hir::LifetimeKind::Error => {}
/* FP:resolve_bound_vars.rs-0883 */             // Those will be resolved by typechecking.
/* FP:resolve_bound_vars.rs-0884 */             hir::LifetimeKind::ImplicitObjectLifetimeDefault | hir::LifetimeKind::Infer => {}
/* FP:resolve_bound_vars.rs-0885 */         }
/* FP:resolve_bound_vars.rs-0886 */     }
/* FP:resolve_bound_vars.rs-0887 */ 
/* FP:resolve_bound_vars.rs-0888 */     fn visit_path(&mut self, path: &hir::Path<'tcx>, hir_id: HirId) {
/* FP:resolve_bound_vars.rs-0889 */         for (i, segment) in path.segments.iter().enumerate() {
/* FP:resolve_bound_vars.rs-0890 */             let depth = path.segments.len() - i - 1;
/* FP:resolve_bound_vars.rs-0891 */             if let Some(args) = segment.args {
/* FP:resolve_bound_vars.rs-0892 */                 self.visit_segment_args(path.res, depth, args);
/* FP:resolve_bound_vars.rs-0893 */             }
/* FP:resolve_bound_vars.rs-0894 */         }
/* FP:resolve_bound_vars.rs-0895 */         if let Res::Def(DefKind::TyParam | DefKind::ConstParam, param_def_id) = path.res {
/* FP:resolve_bound_vars.rs-0896 */             self.resolve_type_ref(param_def_id.expect_local(), hir_id);
/* FP:resolve_bound_vars.rs-0897 */         }
/* FP:resolve_bound_vars.rs-0898 */     }
/* FP:resolve_bound_vars.rs-0899 */ 
/* FP:resolve_bound_vars.rs-0900 */     fn visit_fn(
/* FP:resolve_bound_vars.rs-0901 */         &mut self,
/* FP:resolve_bound_vars.rs-0902 */         fk: intravisit::FnKind<'tcx>,
/* FP:resolve_bound_vars.rs-0903 */         fd: &'tcx hir::FnDecl<'tcx>,
/* FP:resolve_bound_vars.rs-0904 */         body_id: hir::BodyId,
/* FP:resolve_bound_vars.rs-0905 */         _: Span,
/* FP:resolve_bound_vars.rs-0906 */         def_id: LocalDefId,
/* FP:resolve_bound_vars.rs-0907 */     ) {
/* FP:resolve_bound_vars.rs-0908 */         let output = match fd.output {
/* FP:resolve_bound_vars.rs-0909 */             hir::FnRetTy::DefaultReturn(_) => None,
/* FP:resolve_bound_vars.rs-0910 */             hir::FnRetTy::Return(ty) => Some(ty),
/* FP:resolve_bound_vars.rs-0911 */         };
/* FP:resolve_bound_vars.rs-0912 */         if let Some(ty) = output
/* FP:resolve_bound_vars.rs-0913 */             && let hir::TyKind::InferDelegation(sig_id, _) = ty.kind
/* FP:resolve_bound_vars.rs-0914 */         {
/* FP:resolve_bound_vars.rs-0915 */             let bound_vars: Vec<_> =
/* FP:resolve_bound_vars.rs-0916 */                 self.tcx.fn_sig(sig_id).skip_binder().bound_vars().iter().collect();
/* FP:resolve_bound_vars.rs-0917 */             let hir_id = self.tcx.local_def_id_to_hir_id(def_id);
/* FP:resolve_bound_vars.rs-0918 */             self.rbv.late_bound_vars.insert(hir_id.local_id, bound_vars);
/* FP:resolve_bound_vars.rs-0919 */         }
/* FP:resolve_bound_vars.rs-0920 */         self.visit_fn_like_elision(fd.inputs, output, matches!(fk, intravisit::FnKind::Closure));
/* FP:resolve_bound_vars.rs-0921 */         intravisit::walk_fn_kind(self, fk);
/* FP:resolve_bound_vars.rs-0922 */         self.visit_nested_body(body_id)
/* FP:resolve_bound_vars.rs-0923 */     }
/* FP:resolve_bound_vars.rs-0924 */ 
/* FP:resolve_bound_vars.rs-0925 */     fn visit_generics(&mut self, generics: &'tcx hir::Generics<'tcx>) {
/* FP:resolve_bound_vars.rs-0926 */         let scope = Scope::TraitRefBoundary { s: self.scope };
/* FP:resolve_bound_vars.rs-0927 */         self.with(scope, |this| {
/* FP:resolve_bound_vars.rs-0928 */             walk_list!(this, visit_generic_param, generics.params);
/* FP:resolve_bound_vars.rs-0929 */             walk_list!(this, visit_where_predicate, generics.predicates);
/* FP:resolve_bound_vars.rs-0930 */         })
/* FP:resolve_bound_vars.rs-0931 */     }
/* FP:resolve_bound_vars.rs-0932 */ 
/* FP:resolve_bound_vars.rs-0933 */     fn visit_where_predicate(&mut self, predicate: &'tcx hir::WherePredicate<'tcx>) {
/* FP:resolve_bound_vars.rs-0934 */         let hir_id = predicate.hir_id;
/* FP:resolve_bound_vars.rs-0935 */         match predicate.kind {
/* FP:resolve_bound_vars.rs-0936 */             &hir::WherePredicateKind::BoundPredicate(hir::WhereBoundPredicate {
/* FP:resolve_bound_vars.rs-0937 */                 bounded_ty,
/* FP:resolve_bound_vars.rs-0938 */                 bounds,
/* FP:resolve_bound_vars.rs-0939 */                 bound_generic_params,
/* FP:resolve_bound_vars.rs-0940 */                 origin,
/* FP:resolve_bound_vars.rs-0941 */                 ..
/* FP:resolve_bound_vars.rs-0942 */             }) => {
/* FP:resolve_bound_vars.rs-0943 */                 let (bound_vars, binders): (FxIndexMap<LocalDefId, ResolvedArg>, Vec<_>) =
/* FP:resolve_bound_vars.rs-0944 */                     bound_generic_params
/* FP:resolve_bound_vars.rs-0945 */                         .iter()
/* FP:resolve_bound_vars.rs-0946 */                         .enumerate()
/* FP:resolve_bound_vars.rs-0947 */                         .map(|(late_bound_idx, param)| {
/* FP:resolve_bound_vars.rs-0948 */                             (
/* FP:resolve_bound_vars.rs-0949 */                                 (param.def_id, ResolvedArg::late(late_bound_idx as u32, param)),
/* FP:resolve_bound_vars.rs-0950 */                                 late_arg_as_bound_arg(param),
/* FP:resolve_bound_vars.rs-0951 */                             )
/* FP:resolve_bound_vars.rs-0952 */                         })
/* FP:resolve_bound_vars.rs-0953 */                         .unzip();
/* FP:resolve_bound_vars.rs-0954 */ 
/* FP:resolve_bound_vars.rs-0955 */                 self.record_late_bound_vars(hir_id, binders);
/* FP:resolve_bound_vars.rs-0956 */ 
/* FP:resolve_bound_vars.rs-0957 */                 // If this is an RTN type in the self type, then append those to the binder.
/* FP:resolve_bound_vars.rs-0958 */                 self.try_append_return_type_notation_params(hir_id, bounded_ty);
/* FP:resolve_bound_vars.rs-0959 */ 
/* FP:resolve_bound_vars.rs-0960 */                 // Even if there are no lifetimes defined here, we still wrap it in a binder
/* FP:resolve_bound_vars.rs-0961 */                 // scope. If there happens to be a nested poly trait ref (an error), that
/* FP:resolve_bound_vars.rs-0962 */                 // will be `Concatenating` anyways, so we don't have to worry about the depth
/* FP:resolve_bound_vars.rs-0963 */                 // being wrong.
/* FP:resolve_bound_vars.rs-0964 */                 let scope = Scope::Binder {
/* FP:resolve_bound_vars.rs-0965 */                     hir_id,
/* FP:resolve_bound_vars.rs-0966 */                     bound_vars,
/* FP:resolve_bound_vars.rs-0967 */                     s: self.scope,
/* FP:resolve_bound_vars.rs-0968 */                     scope_type: BinderScopeType::Normal,
/* FP:resolve_bound_vars.rs-0969 */                     where_bound_origin: Some(origin),
/* FP:resolve_bound_vars.rs-0970 */                 };
/* FP:resolve_bound_vars.rs-0971 */                 self.with(scope, |this| {
/* FP:resolve_bound_vars.rs-0972 */                     walk_list!(this, visit_generic_param, bound_generic_params);
/* FP:resolve_bound_vars.rs-0973 */                     this.visit_ty_unambig(bounded_ty);
/* FP:resolve_bound_vars.rs-0974 */                     walk_list!(this, visit_param_bound, bounds);
/* FP:resolve_bound_vars.rs-0975 */                 })
/* FP:resolve_bound_vars.rs-0976 */             }
/* FP:resolve_bound_vars.rs-0977 */             &hir::WherePredicateKind::RegionPredicate(hir::WhereRegionPredicate {
/* FP:resolve_bound_vars.rs-0978 */                 lifetime,
/* FP:resolve_bound_vars.rs-0979 */                 bounds,
/* FP:resolve_bound_vars.rs-0980 */                 ..
/* FP:resolve_bound_vars.rs-0981 */             }) => {
/* FP:resolve_bound_vars.rs-0982 */                 self.visit_lifetime(lifetime);
/* FP:resolve_bound_vars.rs-0983 */                 walk_list!(self, visit_param_bound, bounds);
/* FP:resolve_bound_vars.rs-0984 */             }
/* FP:resolve_bound_vars.rs-0985 */             &hir::WherePredicateKind::EqPredicate(hir::WhereEqPredicate {
/* FP:resolve_bound_vars.rs-0986 */                 lhs_ty, rhs_ty, ..
/* FP:resolve_bound_vars.rs-0987 */             }) => {
/* FP:resolve_bound_vars.rs-0988 */                 self.visit_ty_unambig(lhs_ty);
/* FP:resolve_bound_vars.rs-0989 */                 self.visit_ty_unambig(rhs_ty);
/* FP:resolve_bound_vars.rs-0990 */             }
/* FP:resolve_bound_vars.rs-0991 */         }
/* FP:resolve_bound_vars.rs-0992 */     }
/* FP:resolve_bound_vars.rs-0993 */ 
/* FP:resolve_bound_vars.rs-0994 */     fn visit_poly_trait_ref(&mut self, trait_ref: &'tcx hir::PolyTraitRef<'tcx>) {
/* FP:resolve_bound_vars.rs-0995 */         self.visit_poly_trait_ref_inner(trait_ref, NonLifetimeBinderAllowed::Allow);
/* FP:resolve_bound_vars.rs-0996 */     }
/* FP:resolve_bound_vars.rs-0997 */ 
/* FP:resolve_bound_vars.rs-0998 */     fn visit_anon_const(&mut self, c: &'tcx hir::AnonConst) {
/* FP:resolve_bound_vars.rs-0999 */         self.with(
/* FP:resolve_bound_vars.rs-1000 */             Scope::LateBoundary { s: self.scope, what: "constant", deny_late_regions: true },
/* FP:resolve_bound_vars.rs-1001 */             |this| {
/* FP:resolve_bound_vars.rs-1002 */                 intravisit::walk_anon_const(this, c);
/* FP:resolve_bound_vars.rs-1003 */             },
/* FP:resolve_bound_vars.rs-1004 */         );
/* FP:resolve_bound_vars.rs-1005 */     }
/* FP:resolve_bound_vars.rs-1006 */ 
/* FP:resolve_bound_vars.rs-1007 */     fn visit_generic_param(&mut self, p: &'tcx GenericParam<'tcx>) {
/* FP:resolve_bound_vars.rs-1008 */         match p.kind {
/* FP:resolve_bound_vars.rs-1009 */             GenericParamKind::Type { .. } | GenericParamKind::Const { .. } => {
/* FP:resolve_bound_vars.rs-1010 */                 self.resolve_type_ref(p.def_id, p.hir_id);
/* FP:resolve_bound_vars.rs-1011 */             }
/* FP:resolve_bound_vars.rs-1012 */             GenericParamKind::Lifetime { .. } => {
/* FP:resolve_bound_vars.rs-1013 */                 // No need to resolve lifetime params, we don't use them for things
/* FP:resolve_bound_vars.rs-1014 */                 // like implicit `?Sized` or const-param-has-ty predicates.
/* FP:resolve_bound_vars.rs-1015 */             }
/* FP:resolve_bound_vars.rs-1016 */         }
/* FP:resolve_bound_vars.rs-1017 */ 
/* FP:resolve_bound_vars.rs-1018 */         match p.kind {
/* FP:resolve_bound_vars.rs-1019 */             GenericParamKind::Lifetime { .. } => {}
/* FP:resolve_bound_vars.rs-1020 */             GenericParamKind::Type { default, .. } => {
/* FP:resolve_bound_vars.rs-1021 */                 if let Some(ty) = default {
/* FP:resolve_bound_vars.rs-1022 */                     self.visit_ty_unambig(ty);
/* FP:resolve_bound_vars.rs-1023 */                 }
/* FP:resolve_bound_vars.rs-1024 */             }
/* FP:resolve_bound_vars.rs-1025 */             GenericParamKind::Const { ty, default, .. } => {
/* FP:resolve_bound_vars.rs-1026 */                 self.visit_ty_unambig(ty);
/* FP:resolve_bound_vars.rs-1027 */                 if let Some(default) = default {
/* FP:resolve_bound_vars.rs-1028 */                     self.visit_const_arg_unambig(default);
/* FP:resolve_bound_vars.rs-1029 */                 }
/* FP:resolve_bound_vars.rs-1030 */             }
/* FP:resolve_bound_vars.rs-1031 */         }
/* FP:resolve_bound_vars.rs-1032 */     }
/* FP:resolve_bound_vars.rs-1033 */ }
/* FP:resolve_bound_vars.rs-1034 */ 
/* FP:resolve_bound_vars.rs-1035 */ fn object_lifetime_default(tcx: TyCtxt<'_>, param_def_id: LocalDefId) -> ObjectLifetimeDefault {
/* FP:resolve_bound_vars.rs-1036 */     debug_assert_eq!(tcx.def_kind(param_def_id), DefKind::TyParam);
/* FP:resolve_bound_vars.rs-1037 */     let hir::Node::GenericParam(param) = tcx.hir_node_by_def_id(param_def_id) else {
/* FP:resolve_bound_vars.rs-1038 */         bug!("expected GenericParam for object_lifetime_default");
/* FP:resolve_bound_vars.rs-1039 */     };
/* FP:resolve_bound_vars.rs-1040 */     match param.source {
/* FP:resolve_bound_vars.rs-1041 */         hir::GenericParamSource::Generics => {
/* FP:resolve_bound_vars.rs-1042 */             let parent_def_id = tcx.local_parent(param_def_id);
/* FP:resolve_bound_vars.rs-1043 */             let generics = tcx.hir_get_generics(parent_def_id).unwrap();
/* FP:resolve_bound_vars.rs-1044 */             let param_hir_id = tcx.local_def_id_to_hir_id(param_def_id);
/* FP:resolve_bound_vars.rs-1045 */             let param = generics.params.iter().find(|p| p.hir_id == param_hir_id).unwrap();
/* FP:resolve_bound_vars.rs-1046 */ 
/* FP:resolve_bound_vars.rs-1047 */             // Scan the bounds and where-clauses on parameters to extract bounds
/* FP:resolve_bound_vars.rs-1048 */             // of the form `T:'a` so as to determine the `ObjectLifetimeDefault`
/* FP:resolve_bound_vars.rs-1049 */             // for each type parameter.
/* FP:resolve_bound_vars.rs-1050 */             match param.kind {
/* FP:resolve_bound_vars.rs-1051 */                 GenericParamKind::Type { .. } => {
/* FP:resolve_bound_vars.rs-1052 */                     let mut set = Set1::Empty;
/* FP:resolve_bound_vars.rs-1053 */ 
/* FP:resolve_bound_vars.rs-1054 */                     // Look for `type: ...` where clauses.
/* FP:resolve_bound_vars.rs-1055 */                     for bound in generics.bounds_for_param(param_def_id) {
/* FP:resolve_bound_vars.rs-1056 */                         // Ignore `for<'a> type: ...` as they can change what
/* FP:resolve_bound_vars.rs-1057 */                         // lifetimes mean (although we could "just" handle it).
/* FP:resolve_bound_vars.rs-1058 */                         if !bound.bound_generic_params.is_empty() {
/* FP:resolve_bound_vars.rs-1059 */                             continue;
/* FP:resolve_bound_vars.rs-1060 */                         }
/* FP:resolve_bound_vars.rs-1061 */ 
/* FP:resolve_bound_vars.rs-1062 */                         for bound in bound.bounds {
/* FP:resolve_bound_vars.rs-1063 */                             if let hir::GenericBound::Outlives(lifetime) = bound {
/* FP:resolve_bound_vars.rs-1064 */                                 set.insert(lifetime.kind);
/* FP:resolve_bound_vars.rs-1065 */                             }
/* FP:resolve_bound_vars.rs-1066 */                         }
/* FP:resolve_bound_vars.rs-1067 */                     }
/* FP:resolve_bound_vars.rs-1068 */ 
/* FP:resolve_bound_vars.rs-1069 */                     match set {
/* FP:resolve_bound_vars.rs-1070 */                         Set1::Empty => ObjectLifetimeDefault::Empty,
/* FP:resolve_bound_vars.rs-1071 */                         Set1::One(hir::LifetimeKind::Static) => ObjectLifetimeDefault::Static,
/* FP:resolve_bound_vars.rs-1072 */                         Set1::One(hir::LifetimeKind::Param(param_def_id)) => {
/* FP:resolve_bound_vars.rs-1073 */                             ObjectLifetimeDefault::Param(param_def_id.to_def_id())
/* FP:resolve_bound_vars.rs-1074 */                         }
/* FP:resolve_bound_vars.rs-1075 */                         _ => ObjectLifetimeDefault::Ambiguous,
/* FP:resolve_bound_vars.rs-1076 */                     }
/* FP:resolve_bound_vars.rs-1077 */                 }
/* FP:resolve_bound_vars.rs-1078 */                 _ => {
/* FP:resolve_bound_vars.rs-1079 */                     bug!("object_lifetime_default_raw must only be called on a type parameter")
/* FP:resolve_bound_vars.rs-1080 */                 }
/* FP:resolve_bound_vars.rs-1081 */             }
/* FP:resolve_bound_vars.rs-1082 */         }
/* FP:resolve_bound_vars.rs-1083 */         hir::GenericParamSource::Binder => ObjectLifetimeDefault::Empty,
/* FP:resolve_bound_vars.rs-1084 */     }
/* FP:resolve_bound_vars.rs-1085 */ }
/* FP:resolve_bound_vars.rs-1086 */ 
/* FP:resolve_bound_vars.rs-1087 */ impl<'a, 'tcx> BoundVarContext<'a, 'tcx> {
/* FP:resolve_bound_vars.rs-1088 */     fn with<F>(&mut self, wrap_scope: Scope<'_>, f: F)
/* FP:resolve_bound_vars.rs-1089 */     where
/* FP:resolve_bound_vars.rs-1090 */         F: for<'b> FnOnce(&mut BoundVarContext<'b, 'tcx>),
/* FP:resolve_bound_vars.rs-1091 */     {
/* FP:resolve_bound_vars.rs-1092 */         let BoundVarContext { tcx, rbv, disambiguator, .. } = self;
/* FP:resolve_bound_vars.rs-1093 */         let mut this = BoundVarContext { tcx: *tcx, rbv, disambiguator, scope: &wrap_scope };
/* FP:resolve_bound_vars.rs-1094 */         let span = debug_span!("scope", scope = ?this.scope.debug_truncated());
/* FP:resolve_bound_vars.rs-1095 */         {
/* FP:resolve_bound_vars.rs-1096 */             let _enter = span.enter();
/* FP:resolve_bound_vars.rs-1097 */             f(&mut this);
/* FP:resolve_bound_vars.rs-1098 */         }
/* FP:resolve_bound_vars.rs-1099 */     }
/* FP:resolve_bound_vars.rs-1100 */ 
/* FP:resolve_bound_vars.rs-1101 */     fn record_late_bound_vars(&mut self, hir_id: HirId, binder: Vec<ty::BoundVariableKind>) {
/* FP:resolve_bound_vars.rs-1102 */         if let Some(old) = self.rbv.late_bound_vars.insert(hir_id.local_id, binder) {
/* FP:resolve_bound_vars.rs-1103 */             bug!(
/* FP:resolve_bound_vars.rs-1104 */                 "overwrote bound vars for {hir_id:?}:\nold={old:?}\nnew={:?}",
/* FP:resolve_bound_vars.rs-1105 */                 self.rbv.late_bound_vars[&hir_id.local_id]
/* FP:resolve_bound_vars.rs-1106 */             )
/* FP:resolve_bound_vars.rs-1107 */         }
/* FP:resolve_bound_vars.rs-1108 */     }
/* FP:resolve_bound_vars.rs-1109 */ 
/* FP:resolve_bound_vars.rs-1110 */     /// Visits self by adding a scope and handling recursive walk over the contents with `walk`.
/* FP:resolve_bound_vars.rs-1111 */     ///
/* FP:resolve_bound_vars.rs-1112 */     /// Handles visiting fns and methods. These are a bit complicated because we must distinguish
/* FP:resolve_bound_vars.rs-1113 */     /// early- vs late-bound lifetime parameters. We do this by checking which lifetimes appear
/* FP:resolve_bound_vars.rs-1114 */     /// within type bounds; those are early bound lifetimes, and the rest are late bound.
/* FP:resolve_bound_vars.rs-1115 */     ///
/* FP:resolve_bound_vars.rs-1116 */     /// For example:
/* FP:resolve_bound_vars.rs-1117 */     ///
/* FP:resolve_bound_vars.rs-1118 */     ///    fn foo<'a,'b,'c,T:Trait<'b>>(...)
/* FP:resolve_bound_vars.rs-1119 */     ///
/* FP:resolve_bound_vars.rs-1120 */     /// Here `'a` and `'c` are late bound but `'b` is early bound. Note that early- and late-bound
/* FP:resolve_bound_vars.rs-1121 */     /// lifetimes may be interspersed together.
/* FP:resolve_bound_vars.rs-1122 */     ///
/* FP:resolve_bound_vars.rs-1123 */     /// If early bound lifetimes are present, we separate them into their own list (and likewise
/* FP:resolve_bound_vars.rs-1124 */     /// for late bound). They will be numbered sequentially, starting from the lowest index that is
/* FP:resolve_bound_vars.rs-1125 */     /// already in scope (for a fn item, that will be 0, but for a method it might not be). Late
/* FP:resolve_bound_vars.rs-1126 */     /// bound lifetimes are resolved by name and associated with a binder ID (`binder_id`), so the
/* FP:resolve_bound_vars.rs-1127 */     /// ordering is not important there.
/* FP:resolve_bound_vars.rs-1128 */     fn visit_early_late<F>(&mut self, hir_id: HirId, generics: &'tcx hir::Generics<'tcx>, walk: F)
/* FP:resolve_bound_vars.rs-1129 */     where
/* FP:resolve_bound_vars.rs-1130 */         F: for<'b, 'c> FnOnce(&'b mut BoundVarContext<'c, 'tcx>),
/* FP:resolve_bound_vars.rs-1131 */     {
/* FP:resolve_bound_vars.rs-1132 */         let mut named_late_bound_vars = 0;
/* FP:resolve_bound_vars.rs-1133 */         let bound_vars: FxIndexMap<LocalDefId, ResolvedArg> = generics
/* FP:resolve_bound_vars.rs-1134 */             .params
/* FP:resolve_bound_vars.rs-1135 */             .iter()
/* FP:resolve_bound_vars.rs-1136 */             .map(|param| {
/* FP:resolve_bound_vars.rs-1137 */                 (
/* FP:resolve_bound_vars.rs-1138 */                     param.def_id,
/* FP:resolve_bound_vars.rs-1139 */                     match param.kind {
/* FP:resolve_bound_vars.rs-1140 */                         GenericParamKind::Lifetime { .. } => {
/* FP:resolve_bound_vars.rs-1141 */                             if self.tcx.is_late_bound(param.hir_id) {
/* FP:resolve_bound_vars.rs-1142 */                                 let late_bound_idx = named_late_bound_vars;
/* FP:resolve_bound_vars.rs-1143 */                                 named_late_bound_vars += 1;
/* FP:resolve_bound_vars.rs-1144 */                                 ResolvedArg::late(late_bound_idx, param)
/* FP:resolve_bound_vars.rs-1145 */                             } else {
/* FP:resolve_bound_vars.rs-1146 */                                 ResolvedArg::early(param)
/* FP:resolve_bound_vars.rs-1147 */                             }
/* FP:resolve_bound_vars.rs-1148 */                         }
/* FP:resolve_bound_vars.rs-1149 */                         GenericParamKind::Type { .. } | GenericParamKind::Const { .. } => {
/* FP:resolve_bound_vars.rs-1150 */                             ResolvedArg::early(param)
/* FP:resolve_bound_vars.rs-1151 */                         }
/* FP:resolve_bound_vars.rs-1152 */                     },
/* FP:resolve_bound_vars.rs-1153 */                 )
/* FP:resolve_bound_vars.rs-1154 */             })
/* FP:resolve_bound_vars.rs-1155 */             .collect();
/* FP:resolve_bound_vars.rs-1156 */ 
/* FP:resolve_bound_vars.rs-1157 */         let binders: Vec<_> = generics
/* FP:resolve_bound_vars.rs-1158 */             .params
/* FP:resolve_bound_vars.rs-1159 */             .iter()
/* FP:resolve_bound_vars.rs-1160 */             .filter(|param| {
/* FP:resolve_bound_vars.rs-1161 */                 matches!(param.kind, GenericParamKind::Lifetime { .. })
/* FP:resolve_bound_vars.rs-1162 */                     && self.tcx.is_late_bound(param.hir_id)
/* FP:resolve_bound_vars.rs-1163 */             })
/* FP:resolve_bound_vars.rs-1164 */             .map(|param| late_arg_as_bound_arg(param))
/* FP:resolve_bound_vars.rs-1165 */             .collect();
/* FP:resolve_bound_vars.rs-1166 */         self.record_late_bound_vars(hir_id, binders);
/* FP:resolve_bound_vars.rs-1167 */         let scope = Scope::Binder {
/* FP:resolve_bound_vars.rs-1168 */             hir_id,
/* FP:resolve_bound_vars.rs-1169 */             bound_vars,
/* FP:resolve_bound_vars.rs-1170 */             s: self.scope,
/* FP:resolve_bound_vars.rs-1171 */             scope_type: BinderScopeType::Normal,
/* FP:resolve_bound_vars.rs-1172 */             where_bound_origin: None,
/* FP:resolve_bound_vars.rs-1173 */         };
/* FP:resolve_bound_vars.rs-1174 */         self.with(scope, walk);
/* FP:resolve_bound_vars.rs-1175 */     }
/* FP:resolve_bound_vars.rs-1176 */ 
/* FP:resolve_bound_vars.rs-1177 */     fn visit_early<F>(&mut self, hir_id: HirId, generics: &'tcx hir::Generics<'tcx>, walk: F)
/* FP:resolve_bound_vars.rs-1178 */     where
/* FP:resolve_bound_vars.rs-1179 */         F: for<'b, 'c> FnOnce(&'b mut BoundVarContext<'c, 'tcx>),
/* FP:resolve_bound_vars.rs-1180 */     {
/* FP:resolve_bound_vars.rs-1181 */         let bound_vars =
/* FP:resolve_bound_vars.rs-1182 */             generics.params.iter().map(|param| (param.def_id, ResolvedArg::early(param))).collect();
/* FP:resolve_bound_vars.rs-1183 */         self.record_late_bound_vars(hir_id, vec![]);
/* FP:resolve_bound_vars.rs-1184 */         let scope = Scope::Binder {
/* FP:resolve_bound_vars.rs-1185 */             hir_id,
/* FP:resolve_bound_vars.rs-1186 */             bound_vars,
/* FP:resolve_bound_vars.rs-1187 */             s: self.scope,
/* FP:resolve_bound_vars.rs-1188 */             scope_type: BinderScopeType::Normal,
/* FP:resolve_bound_vars.rs-1189 */             where_bound_origin: None,
/* FP:resolve_bound_vars.rs-1190 */         };
/* FP:resolve_bound_vars.rs-1191 */         self.with(scope, |this| {
/* FP:resolve_bound_vars.rs-1192 */             let scope = Scope::TraitRefBoundary { s: this.scope };
/* FP:resolve_bound_vars.rs-1193 */             this.with(scope, walk)
/* FP:resolve_bound_vars.rs-1194 */         });
/* FP:resolve_bound_vars.rs-1195 */     }
/* FP:resolve_bound_vars.rs-1196 */ 
/* FP:resolve_bound_vars.rs-1197 */     #[instrument(level = "debug", skip(self))]
/* FP:resolve_bound_vars.rs-1198 */     fn resolve_lifetime_ref(
/* FP:resolve_bound_vars.rs-1199 */         &mut self,
/* FP:resolve_bound_vars.rs-1200 */         region_def_id: LocalDefId,
/* FP:resolve_bound_vars.rs-1201 */         lifetime_ref: &'tcx hir::Lifetime,
/* FP:resolve_bound_vars.rs-1202 */     ) {
/* FP:resolve_bound_vars.rs-1203 */         // Walk up the scope chain, tracking the number of fn scopes
/* FP:resolve_bound_vars.rs-1204 */         // that we pass through, until we find a lifetime with the
/* FP:resolve_bound_vars.rs-1205 */         // given name or we run out of scopes.
/* FP:resolve_bound_vars.rs-1206 */         // search.
/* FP:resolve_bound_vars.rs-1207 */         let mut late_depth = 0;
/* FP:resolve_bound_vars.rs-1208 */         let mut scope = self.scope;
/* FP:resolve_bound_vars.rs-1209 */         let mut outermost_body = None;
/* FP:resolve_bound_vars.rs-1210 */         let mut crossed_late_boundary = None;
/* FP:resolve_bound_vars.rs-1211 */         let mut opaque_capture_scopes = vec![];
/* FP:resolve_bound_vars.rs-1212 */         let result = loop {
/* FP:resolve_bound_vars.rs-1213 */             match *scope {
/* FP:resolve_bound_vars.rs-1214 */                 Scope::Body { id, s } => {
/* FP:resolve_bound_vars.rs-1215 */                     outermost_body = Some(id);
/* FP:resolve_bound_vars.rs-1216 */                     scope = s;
/* FP:resolve_bound_vars.rs-1217 */                 }
/* FP:resolve_bound_vars.rs-1218 */ 
/* FP:resolve_bound_vars.rs-1219 */                 Scope::Root { opt_parent_item } => {
/* FP:resolve_bound_vars.rs-1220 */                     if let Some(parent_item) = opt_parent_item
/* FP:resolve_bound_vars.rs-1221 */                         && let parent_generics = self.tcx.generics_of(parent_item)
/* FP:resolve_bound_vars.rs-1222 */                         && parent_generics
/* FP:resolve_bound_vars.rs-1223 */                             .param_def_id_to_index(self.tcx, region_def_id.to_def_id())
/* FP:resolve_bound_vars.rs-1224 */                             .is_some()
/* FP:resolve_bound_vars.rs-1225 */                     {
/* FP:resolve_bound_vars.rs-1226 */                         break Some(ResolvedArg::EarlyBound(region_def_id));
/* FP:resolve_bound_vars.rs-1227 */                     }
/* FP:resolve_bound_vars.rs-1228 */                     break None;
/* FP:resolve_bound_vars.rs-1229 */                 }
/* FP:resolve_bound_vars.rs-1230 */ 
/* FP:resolve_bound_vars.rs-1231 */                 Scope::Binder { ref bound_vars, scope_type, s, where_bound_origin, .. } => {
/* FP:resolve_bound_vars.rs-1232 */                     if let Some(&def) = bound_vars.get(&region_def_id) {
/* FP:resolve_bound_vars.rs-1233 */                         break Some(def.shifted(late_depth));
/* FP:resolve_bound_vars.rs-1234 */                     }
/* FP:resolve_bound_vars.rs-1235 */                     match scope_type {
/* FP:resolve_bound_vars.rs-1236 */                         BinderScopeType::Normal => late_depth += 1,
/* FP:resolve_bound_vars.rs-1237 */                         BinderScopeType::Concatenating => {}
/* FP:resolve_bound_vars.rs-1238 */                     }
/* FP:resolve_bound_vars.rs-1239 */                     // Fresh lifetimes in APIT used to be allowed in async fns and forbidden in
/* FP:resolve_bound_vars.rs-1240 */                     // regular fns.
/* FP:resolve_bound_vars.rs-1241 */                     if let Some(hir::PredicateOrigin::ImplTrait) = where_bound_origin
/* FP:resolve_bound_vars.rs-1242 */                         && let hir::LifetimeKind::Param(param_id) = lifetime_ref.kind
/* FP:resolve_bound_vars.rs-1243 */                         && let Some(generics) =
/* FP:resolve_bound_vars.rs-1244 */                             self.tcx.hir_get_generics(self.tcx.local_parent(param_id))
/* FP:resolve_bound_vars.rs-1245 */                         && let Some(param) = generics.params.iter().find(|p| p.def_id == param_id)
/* FP:resolve_bound_vars.rs-1246 */                         && param.is_elided_lifetime()
/* FP:resolve_bound_vars.rs-1247 */                         && !self.tcx.asyncness(lifetime_ref.hir_id.owner.def_id).is_async()
/* FP:resolve_bound_vars.rs-1248 */                         && !self.tcx.features().anonymous_lifetime_in_impl_trait()
/* FP:resolve_bound_vars.rs-1249 */                     {
/* FP:resolve_bound_vars.rs-1250 */                         let mut diag: crate::rustc_errors::Diag<'_> = crate::rustc_session::parse::feature_err(
/* FP:resolve_bound_vars.rs-1251 */                             &self.tcx.sess,
/* FP:resolve_bound_vars.rs-1252 */                             sym::anonymous_lifetime_in_impl_trait,
/* FP:resolve_bound_vars.rs-1253 */                             lifetime_ref.ident.span,
/* FP:resolve_bound_vars.rs-1254 */                             "anonymous lifetimes in `impl Trait` are unstable",
/* FP:resolve_bound_vars.rs-1255 */                         );
/* FP:resolve_bound_vars.rs-1256 */ 
/* FP:resolve_bound_vars.rs-1257 */                         if let Some(generics) =
/* FP:resolve_bound_vars.rs-1258 */                             self.tcx.hir_get_generics(lifetime_ref.hir_id.owner.def_id)
/* FP:resolve_bound_vars.rs-1259 */                         {
/* FP:resolve_bound_vars.rs-1260 */                             let new_param_sugg =
/* FP:resolve_bound_vars.rs-1261 */                                 if let Some(span) = generics.span_for_lifetime_suggestion() {
/* FP:resolve_bound_vars.rs-1262 */                                     (span, "'a, ".to_owned())
/* FP:resolve_bound_vars.rs-1263 */                                 } else {
/* FP:resolve_bound_vars.rs-1264 */                                     (generics.span, "<'a>".to_owned())
/* FP:resolve_bound_vars.rs-1265 */                                 };
/* FP:resolve_bound_vars.rs-1266 */ 
/* FP:resolve_bound_vars.rs-1267 */                             let lifetime_sugg = lifetime_ref.suggestion("'a");
/* FP:resolve_bound_vars.rs-1268 */                             let suggestions = vec![lifetime_sugg, new_param_sugg];
/* FP:resolve_bound_vars.rs-1269 */ 
/* FP:resolve_bound_vars.rs-1270 */                             diag.span_label(
/* FP:resolve_bound_vars.rs-1271 */                                 lifetime_ref.ident.span,
/* FP:resolve_bound_vars.rs-1272 */                                 "expected named lifetime parameter",
/* FP:resolve_bound_vars.rs-1273 */                             );
/* FP:resolve_bound_vars.rs-1274 */                             diag.multipart_suggestion(
/* FP:resolve_bound_vars.rs-1275 */                                 "consider introducing a named lifetime parameter",
/* FP:resolve_bound_vars.rs-1276 */                                 suggestions,
/* FP:resolve_bound_vars.rs-1277 */                                 crate::rustc_errors::Applicability::MaybeIncorrect,
/* FP:resolve_bound_vars.rs-1278 */                             );
/* FP:resolve_bound_vars.rs-1279 */                         }
/* FP:resolve_bound_vars.rs-1280 */ 
/* FP:resolve_bound_vars.rs-1281 */                         diag.emit();
/* FP:resolve_bound_vars.rs-1282 */                         return;
/* FP:resolve_bound_vars.rs-1283 */                     }
/* FP:resolve_bound_vars.rs-1284 */                     scope = s;
/* FP:resolve_bound_vars.rs-1285 */                 }
/* FP:resolve_bound_vars.rs-1286 */ 
/* FP:resolve_bound_vars.rs-1287 */                 Scope::Opaque { captures, def_id, s } => {
/* FP:resolve_bound_vars.rs-1288 */                     opaque_capture_scopes.push((def_id, captures));
/* FP:resolve_bound_vars.rs-1289 */                     late_depth = 0;
/* FP:resolve_bound_vars.rs-1290 */                     scope = s;
/* FP:resolve_bound_vars.rs-1291 */                 }
/* FP:resolve_bound_vars.rs-1292 */ 
/* FP:resolve_bound_vars.rs-1293 */                 Scope::ObjectLifetimeDefault { s, .. }
/* FP:resolve_bound_vars.rs-1294 */                 | Scope::Supertrait { s, .. }
/* FP:resolve_bound_vars.rs-1295 */                 | Scope::TraitRefBoundary { s, .. } => {
/* FP:resolve_bound_vars.rs-1296 */                     scope = s;
/* FP:resolve_bound_vars.rs-1297 */                 }
/* FP:resolve_bound_vars.rs-1298 */ 
/* FP:resolve_bound_vars.rs-1299 */                 Scope::LateBoundary { s, what, deny_late_regions } => {
/* FP:resolve_bound_vars.rs-1300 */                     if deny_late_regions {
/* FP:resolve_bound_vars.rs-1301 */                         crossed_late_boundary = Some(what);
/* FP:resolve_bound_vars.rs-1302 */                     }
/* FP:resolve_bound_vars.rs-1303 */                     scope = s;
/* FP:resolve_bound_vars.rs-1304 */                 }
/* FP:resolve_bound_vars.rs-1305 */             }
/* FP:resolve_bound_vars.rs-1306 */         };
/* FP:resolve_bound_vars.rs-1307 */ 
/* FP:resolve_bound_vars.rs-1308 */         if let Some(mut def) = result {
/* FP:resolve_bound_vars.rs-1309 */             def = self.remap_opaque_captures(&opaque_capture_scopes, def, lifetime_ref.ident);
/* FP:resolve_bound_vars.rs-1310 */ 
/* FP:resolve_bound_vars.rs-1311 */             if let ResolvedArg::EarlyBound(..) = def {
/* FP:resolve_bound_vars.rs-1312 */                 // Do not free early-bound regions, only late-bound ones.
/* FP:resolve_bound_vars.rs-1313 */             } else if let ResolvedArg::LateBound(_, _, param_def_id) = def
/* FP:resolve_bound_vars.rs-1314 */                 && let Some(what) = crossed_late_boundary
/* FP:resolve_bound_vars.rs-1315 */             {
/* FP:resolve_bound_vars.rs-1316 */                 let use_span = lifetime_ref.ident.span;
/* FP:resolve_bound_vars.rs-1317 */                 let def_span = self.tcx.def_span(param_def_id);
/* FP:resolve_bound_vars.rs-1318 */                 let guar = match self.tcx.def_kind(param_def_id) {
/* FP:resolve_bound_vars.rs-1319 */                     DefKind::LifetimeParam => {
/* FP:resolve_bound_vars.rs-1320 */                         self.tcx.dcx().emit_err(errors::CannotCaptureLateBound::Lifetime {
/* FP:resolve_bound_vars.rs-1321 */                             use_span,
/* FP:resolve_bound_vars.rs-1322 */                             def_span,
/* FP:resolve_bound_vars.rs-1323 */                             what,
/* FP:resolve_bound_vars.rs-1324 */                         })
/* FP:resolve_bound_vars.rs-1325 */                     }
/* FP:resolve_bound_vars.rs-1326 */                     kind => span_bug!(
/* FP:resolve_bound_vars.rs-1327 */                         use_span,
/* FP:resolve_bound_vars.rs-1328 */                         "did not expect to resolve lifetime to {}",
/* FP:resolve_bound_vars.rs-1329 */                         kind.descr(param_def_id.to_def_id())
/* FP:resolve_bound_vars.rs-1330 */                     ),
/* FP:resolve_bound_vars.rs-1331 */                 };
/* FP:resolve_bound_vars.rs-1332 */                 def = ResolvedArg::Error(guar);
/* FP:resolve_bound_vars.rs-1333 */             } else if let Some(body_id) = outermost_body {
/* FP:resolve_bound_vars.rs-1334 */                 let fn_id = self.tcx.hir_body_owner(body_id);
/* FP:resolve_bound_vars.rs-1335 */                 match self.tcx.hir_node(fn_id) {
/* FP:resolve_bound_vars.rs-1336 */                     Node::Item(hir::Item { owner_id, kind: hir::ItemKind::Fn { .. }, .. })
/* FP:resolve_bound_vars.rs-1337 */                     | Node::TraitItem(hir::TraitItem {
/* FP:resolve_bound_vars.rs-1338 */                         owner_id,
/* FP:resolve_bound_vars.rs-1339 */                         kind: hir::TraitItemKind::Fn(..),
/* FP:resolve_bound_vars.rs-1340 */                         ..
/* FP:resolve_bound_vars.rs-1341 */                     })
/* FP:resolve_bound_vars.rs-1342 */                     | Node::ImplItem(hir::ImplItem {
/* FP:resolve_bound_vars.rs-1343 */                         owner_id,
/* FP:resolve_bound_vars.rs-1344 */                         kind: hir::ImplItemKind::Fn(..),
/* FP:resolve_bound_vars.rs-1345 */                         ..
/* FP:resolve_bound_vars.rs-1346 */                     }) => {
/* FP:resolve_bound_vars.rs-1347 */                         def = ResolvedArg::Free(owner_id.def_id, def.id().unwrap());
/* FP:resolve_bound_vars.rs-1348 */                     }
/* FP:resolve_bound_vars.rs-1349 */                     Node::Expr(hir::Expr { kind: hir::ExprKind::Closure(closure), .. }) => {
/* FP:resolve_bound_vars.rs-1350 */                         def = ResolvedArg::Free(closure.def_id, def.id().unwrap());
/* FP:resolve_bound_vars.rs-1351 */                     }
/* FP:resolve_bound_vars.rs-1352 */                     _ => {}
/* FP:resolve_bound_vars.rs-1353 */                 }
/* FP:resolve_bound_vars.rs-1354 */             }
/* FP:resolve_bound_vars.rs-1355 */ 
/* FP:resolve_bound_vars.rs-1356 */             self.insert_lifetime(lifetime_ref, def);
/* FP:resolve_bound_vars.rs-1357 */             return;
/* FP:resolve_bound_vars.rs-1358 */         }
/* FP:resolve_bound_vars.rs-1359 */ 
/* FP:resolve_bound_vars.rs-1360 */         // We may fail to resolve higher-ranked lifetimes that are mentioned by APIT.
/* FP:resolve_bound_vars.rs-1361 */         // AST-based resolution does not care for impl-trait desugaring, which are the
/* FP:resolve_bound_vars.rs-1362 */         // responsibility of lowering. This may create a mismatch between the resolution
/* FP:resolve_bound_vars.rs-1363 */         // AST found (`region_def_id`) which points to HRTB, and what HIR allows.
/* FP:resolve_bound_vars.rs-1364 */         // ```
/* FP:resolve_bound_vars.rs-1365 */         // fn foo(x: impl for<'a> Trait<'a, Assoc = impl Copy + 'a>) {}
/* FP:resolve_bound_vars.rs-1366 */         // ```
/* FP:resolve_bound_vars.rs-1367 */         //
/* FP:resolve_bound_vars.rs-1368 */         // In such case, walk back the binders to diagnose it properly.
/* FP:resolve_bound_vars.rs-1369 */         let mut scope = self.scope;
/* FP:resolve_bound_vars.rs-1370 */         loop {
/* FP:resolve_bound_vars.rs-1371 */             match *scope {
/* FP:resolve_bound_vars.rs-1372 */                 Scope::Binder {
/* FP:resolve_bound_vars.rs-1373 */                     where_bound_origin: Some(hir::PredicateOrigin::ImplTrait), ..
/* FP:resolve_bound_vars.rs-1374 */                 } => {
/* FP:resolve_bound_vars.rs-1375 */                     self.tcx.dcx().emit_err(errors::LateBoundInApit::Lifetime {
/* FP:resolve_bound_vars.rs-1376 */                         span: lifetime_ref.ident.span,
/* FP:resolve_bound_vars.rs-1377 */                         param_span: self.tcx.def_span(region_def_id),
/* FP:resolve_bound_vars.rs-1378 */                     });
/* FP:resolve_bound_vars.rs-1379 */                     return;
/* FP:resolve_bound_vars.rs-1380 */                 }
/* FP:resolve_bound_vars.rs-1381 */                 Scope::Root { .. } => break,
/* FP:resolve_bound_vars.rs-1382 */                 Scope::Binder { s, .. }
/* FP:resolve_bound_vars.rs-1383 */                 | Scope::Body { s, .. }
/* FP:resolve_bound_vars.rs-1384 */                 | Scope::Opaque { s, .. }
/* FP:resolve_bound_vars.rs-1385 */                 | Scope::ObjectLifetimeDefault { s, .. }
/* FP:resolve_bound_vars.rs-1386 */                 | Scope::Supertrait { s, .. }
/* FP:resolve_bound_vars.rs-1387 */                 | Scope::TraitRefBoundary { s, .. }
/* FP:resolve_bound_vars.rs-1388 */                 | Scope::LateBoundary { s, .. } => {
/* FP:resolve_bound_vars.rs-1389 */                     scope = s;
/* FP:resolve_bound_vars.rs-1390 */                 }
/* FP:resolve_bound_vars.rs-1391 */             }
/* FP:resolve_bound_vars.rs-1392 */         }
/* FP:resolve_bound_vars.rs-1393 */ 
/* FP:resolve_bound_vars.rs-1394 */         self.tcx.dcx().span_delayed_bug(
/* FP:resolve_bound_vars.rs-1395 */             lifetime_ref.ident.span,
/* FP:resolve_bound_vars.rs-1396 */             format!("Could not resolve {:?} in scope {:#?}", lifetime_ref, self.scope,),
/* FP:resolve_bound_vars.rs-1397 */         );
/* FP:resolve_bound_vars.rs-1398 */     }
/* FP:resolve_bound_vars.rs-1399 */ 
/* FP:resolve_bound_vars.rs-1400 */     /// Check for predicates like `impl for<'a> Trait<impl OtherTrait<'a>>`
/* FP:resolve_bound_vars.rs-1401 */     /// and ban them. Type variables instantiated inside binders aren't
/* FP:resolve_bound_vars.rs-1402 */     /// well-supported at the moment, so this doesn't work.
/* FP:resolve_bound_vars.rs-1403 */     /// In the future, this should be fixed and this error should be removed.
/* FP:resolve_bound_vars.rs-1404 */     fn check_lifetime_is_capturable(
/* FP:resolve_bound_vars.rs-1405 */         &self,
/* FP:resolve_bound_vars.rs-1406 */         opaque_def_id: LocalDefId,
/* FP:resolve_bound_vars.rs-1407 */         lifetime: ResolvedArg,
/* FP:resolve_bound_vars.rs-1408 */         capture_span: Span,
/* FP:resolve_bound_vars.rs-1409 */     ) -> Result<(), ErrorGuaranteed> {
/* FP:resolve_bound_vars.rs-1410 */         let ResolvedArg::LateBound(_, _, lifetime_def_id) = lifetime else { return Ok(()) };
/* FP:resolve_bound_vars.rs-1411 */         let lifetime_hir_id = self.tcx.local_def_id_to_hir_id(lifetime_def_id);
/* FP:resolve_bound_vars.rs-1412 */         let bad_place = match self.tcx.hir_node(self.tcx.parent_hir_id(lifetime_hir_id)) {
/* FP:resolve_bound_vars.rs-1413 */             // Opaques do not declare their own lifetimes, so if a lifetime comes from an opaque
/* FP:resolve_bound_vars.rs-1414 */             // it must be a reified late-bound lifetime from a trait goal.
/* FP:resolve_bound_vars.rs-1415 */             hir::Node::OpaqueTy(_) => "higher-ranked lifetime from outer `impl Trait`",
/* FP:resolve_bound_vars.rs-1416 */             // Other items are fine.
/* FP:resolve_bound_vars.rs-1417 */             hir::Node::Item(_) | hir::Node::TraitItem(_) | hir::Node::ImplItem(_) => return Ok(()),
/* FP:resolve_bound_vars.rs-1418 */             hir::Node::Ty(hir::Ty { kind: hir::TyKind::FnPtr(_), .. }) => {
/* FP:resolve_bound_vars.rs-1419 */                 "higher-ranked lifetime from function pointer"
/* FP:resolve_bound_vars.rs-1420 */             }
/* FP:resolve_bound_vars.rs-1421 */             hir::Node::Ty(hir::Ty { kind: hir::TyKind::TraitObject(..), .. }) => {
/* FP:resolve_bound_vars.rs-1422 */                 "higher-ranked lifetime from `dyn` type"
/* FP:resolve_bound_vars.rs-1423 */             }
/* FP:resolve_bound_vars.rs-1424 */             _ => "higher-ranked lifetime",
/* FP:resolve_bound_vars.rs-1425 */         };
/* FP:resolve_bound_vars.rs-1426 */ 
/* FP:resolve_bound_vars.rs-1427 */         let decl_span = self.tcx.def_span(lifetime_def_id);
/* FP:resolve_bound_vars.rs-1428 */         let (span, label) = if capture_span != decl_span {
/* FP:resolve_bound_vars.rs-1429 */             (capture_span, None)
/* FP:resolve_bound_vars.rs-1430 */         } else {
/* FP:resolve_bound_vars.rs-1431 */             let opaque_span = self.tcx.def_span(opaque_def_id);
/* FP:resolve_bound_vars.rs-1432 */             (opaque_span, Some(opaque_span))
/* FP:resolve_bound_vars.rs-1433 */         };
/* FP:resolve_bound_vars.rs-1434 */ 
/* FP:resolve_bound_vars.rs-1435 */         // Ensure that the parent of the def is an item, not HRTB
/* FP:resolve_bound_vars.rs-1436 */         let guar = self.tcx.dcx().emit_err(errors::OpaqueCapturesHigherRankedLifetime {
/* FP:resolve_bound_vars.rs-1437 */             span,
/* FP:resolve_bound_vars.rs-1438 */             label,
/* FP:resolve_bound_vars.rs-1439 */             decl_span,
/* FP:resolve_bound_vars.rs-1440 */             bad_place,
/* FP:resolve_bound_vars.rs-1441 */         });
/* FP:resolve_bound_vars.rs-1442 */         Err(guar)
/* FP:resolve_bound_vars.rs-1443 */     }
/* FP:resolve_bound_vars.rs-1444 */ 
/* FP:resolve_bound_vars.rs-1445 */     #[instrument(level = "trace", skip(self, opaque_capture_scopes), ret)]
/* FP:resolve_bound_vars.rs-1446 */     fn remap_opaque_captures(
/* FP:resolve_bound_vars.rs-1447 */         &mut self,
/* FP:resolve_bound_vars.rs-1448 */         opaque_capture_scopes: &Vec<(LocalDefId, &RefCell<FxIndexMap<ResolvedArg, LocalDefId>>)>,
/* FP:resolve_bound_vars.rs-1449 */         mut lifetime: ResolvedArg,
/* FP:resolve_bound_vars.rs-1450 */         ident: Ident,
/* FP:resolve_bound_vars.rs-1451 */     ) -> ResolvedArg {
/* FP:resolve_bound_vars.rs-1452 */         if let Some(&(opaque_def_id, _)) = opaque_capture_scopes.last() {
/* FP:resolve_bound_vars.rs-1453 */             if let Err(guar) =
/* FP:resolve_bound_vars.rs-1454 */                 self.check_lifetime_is_capturable(opaque_def_id, lifetime, ident.span)
/* FP:resolve_bound_vars.rs-1455 */             {
/* FP:resolve_bound_vars.rs-1456 */                 lifetime = ResolvedArg::Error(guar);
/* FP:resolve_bound_vars.rs-1457 */             }
/* FP:resolve_bound_vars.rs-1458 */         }
/* FP:resolve_bound_vars.rs-1459 */ 
/* FP:resolve_bound_vars.rs-1460 */         for &(opaque_def_id, captures) in opaque_capture_scopes.iter().rev() {
/* FP:resolve_bound_vars.rs-1461 */             let mut captures = captures.borrow_mut();
/* FP:resolve_bound_vars.rs-1462 */             let remapped = *captures.entry(lifetime).or_insert_with(|| {
/* FP:resolve_bound_vars.rs-1463 */                 // `opaque_def_id` is unique to the `BoundVarContext` pass which is executed once
/* FP:resolve_bound_vars.rs-1464 */                 // per `resolve_bound_vars` query. This is the only location that creates
/* FP:resolve_bound_vars.rs-1465 */                 // `OpaqueLifetime` paths. `<opaque_def_id>::OpaqueLifetime(..)` is thus unique
/* FP:resolve_bound_vars.rs-1466 */                 // to this query and duplicates within the query are handled by `self.disambiguator`.
/* FP:resolve_bound_vars.rs-1467 */                 let feed = self.tcx.create_def(
/* FP:resolve_bound_vars.rs-1468 */                     opaque_def_id,
/* FP:resolve_bound_vars.rs-1469 */                     None,
/* FP:resolve_bound_vars.rs-1470 */                     DefKind::LifetimeParam,
/* FP:resolve_bound_vars.rs-1471 */                     Some(DefPathData::OpaqueLifetime(ident.name)),
/* FP:resolve_bound_vars.rs-1472 */                     &mut self.disambiguator,
/* FP:resolve_bound_vars.rs-1473 */                 );
/* FP:resolve_bound_vars.rs-1474 */                 feed.def_span(ident.span);
/* FP:resolve_bound_vars.rs-1475 */                 feed.def_ident_span(Some(ident.span));
/* FP:resolve_bound_vars.rs-1476 */                 feed.def_id()
/* FP:resolve_bound_vars.rs-1477 */             });
/* FP:resolve_bound_vars.rs-1478 */             lifetime = ResolvedArg::EarlyBound(remapped);
/* FP:resolve_bound_vars.rs-1479 */         }
/* FP:resolve_bound_vars.rs-1480 */         lifetime
/* FP:resolve_bound_vars.rs-1481 */     }
/* FP:resolve_bound_vars.rs-1482 */ 
/* FP:resolve_bound_vars.rs-1483 */     fn resolve_type_ref(&mut self, param_def_id: LocalDefId, hir_id: HirId) {
/* FP:resolve_bound_vars.rs-1484 */         // Walk up the scope chain, tracking the number of fn scopes
/* FP:resolve_bound_vars.rs-1485 */         // that we pass through, until we find a lifetime with the
/* FP:resolve_bound_vars.rs-1486 */         // given name or we run out of scopes.
/* FP:resolve_bound_vars.rs-1487 */         // search.
/* FP:resolve_bound_vars.rs-1488 */         let mut late_depth = 0;
/* FP:resolve_bound_vars.rs-1489 */         let mut scope = self.scope;
/* FP:resolve_bound_vars.rs-1490 */         let mut crossed_late_boundary = None;
/* FP:resolve_bound_vars.rs-1491 */ 
/* FP:resolve_bound_vars.rs-1492 */         let result = loop {
/* FP:resolve_bound_vars.rs-1493 */             match *scope {
/* FP:resolve_bound_vars.rs-1494 */                 Scope::Body { s, .. } => {
/* FP:resolve_bound_vars.rs-1495 */                     scope = s;
/* FP:resolve_bound_vars.rs-1496 */                 }
/* FP:resolve_bound_vars.rs-1497 */ 
/* FP:resolve_bound_vars.rs-1498 */                 Scope::Root { opt_parent_item } => {
/* FP:resolve_bound_vars.rs-1499 */                     if let Some(parent_item) = opt_parent_item
/* FP:resolve_bound_vars.rs-1500 */                         && let parent_generics = self.tcx.generics_of(parent_item)
/* FP:resolve_bound_vars.rs-1501 */                         && parent_generics
/* FP:resolve_bound_vars.rs-1502 */                             .param_def_id_to_index(self.tcx, param_def_id.to_def_id())
/* FP:resolve_bound_vars.rs-1503 */                             .is_some()
/* FP:resolve_bound_vars.rs-1504 */                     {
/* FP:resolve_bound_vars.rs-1505 */                         break Some(ResolvedArg::EarlyBound(param_def_id));
/* FP:resolve_bound_vars.rs-1506 */                     }
/* FP:resolve_bound_vars.rs-1507 */                     break None;
/* FP:resolve_bound_vars.rs-1508 */                 }
/* FP:resolve_bound_vars.rs-1509 */ 
/* FP:resolve_bound_vars.rs-1510 */                 Scope::Binder { ref bound_vars, scope_type, s, .. } => {
/* FP:resolve_bound_vars.rs-1511 */                     if let Some(&def) = bound_vars.get(&param_def_id) {
/* FP:resolve_bound_vars.rs-1512 */                         break Some(def.shifted(late_depth));
/* FP:resolve_bound_vars.rs-1513 */                     }
/* FP:resolve_bound_vars.rs-1514 */                     match scope_type {
/* FP:resolve_bound_vars.rs-1515 */                         BinderScopeType::Normal => late_depth += 1,
/* FP:resolve_bound_vars.rs-1516 */                         BinderScopeType::Concatenating => {}
/* FP:resolve_bound_vars.rs-1517 */                     }
/* FP:resolve_bound_vars.rs-1518 */                     scope = s;
/* FP:resolve_bound_vars.rs-1519 */                 }
/* FP:resolve_bound_vars.rs-1520 */ 
/* FP:resolve_bound_vars.rs-1521 */                 Scope::ObjectLifetimeDefault { s, .. }
/* FP:resolve_bound_vars.rs-1522 */                 | Scope::Opaque { s, .. }
/* FP:resolve_bound_vars.rs-1523 */                 | Scope::Supertrait { s, .. }
/* FP:resolve_bound_vars.rs-1524 */                 | Scope::TraitRefBoundary { s, .. } => {
/* FP:resolve_bound_vars.rs-1525 */                     scope = s;
/* FP:resolve_bound_vars.rs-1526 */                 }
/* FP:resolve_bound_vars.rs-1527 */ 
/* FP:resolve_bound_vars.rs-1528 */                 Scope::LateBoundary { s, what, deny_late_regions: _ } => {
/* FP:resolve_bound_vars.rs-1529 */                     crossed_late_boundary = Some(what);
/* FP:resolve_bound_vars.rs-1530 */                     scope = s;
/* FP:resolve_bound_vars.rs-1531 */                 }
/* FP:resolve_bound_vars.rs-1532 */             }
/* FP:resolve_bound_vars.rs-1533 */         };
/* FP:resolve_bound_vars.rs-1534 */ 
/* FP:resolve_bound_vars.rs-1535 */         if let Some(def) = result {
/* FP:resolve_bound_vars.rs-1536 */             if let ResolvedArg::LateBound(..) = def
/* FP:resolve_bound_vars.rs-1537 */                 && let Some(what) = crossed_late_boundary
/* FP:resolve_bound_vars.rs-1538 */             {
/* FP:resolve_bound_vars.rs-1539 */                 let use_span = self.tcx.hir_span(hir_id);
/* FP:resolve_bound_vars.rs-1540 */                 let def_span = self.tcx.def_span(param_def_id);
/* FP:resolve_bound_vars.rs-1541 */                 let guar = match self.tcx.def_kind(param_def_id) {
/* FP:resolve_bound_vars.rs-1542 */                     DefKind::ConstParam => {
/* FP:resolve_bound_vars.rs-1543 */                         self.tcx.dcx().emit_err(errors::CannotCaptureLateBound::Const {
/* FP:resolve_bound_vars.rs-1544 */                             use_span,
/* FP:resolve_bound_vars.rs-1545 */                             def_span,
/* FP:resolve_bound_vars.rs-1546 */                             what,
/* FP:resolve_bound_vars.rs-1547 */                         })
/* FP:resolve_bound_vars.rs-1548 */                     }
/* FP:resolve_bound_vars.rs-1549 */                     DefKind::TyParam => {
/* FP:resolve_bound_vars.rs-1550 */                         self.tcx.dcx().emit_err(errors::CannotCaptureLateBound::Type {
/* FP:resolve_bound_vars.rs-1551 */                             use_span,
/* FP:resolve_bound_vars.rs-1552 */                             def_span,
/* FP:resolve_bound_vars.rs-1553 */                             what,
/* FP:resolve_bound_vars.rs-1554 */                         })
/* FP:resolve_bound_vars.rs-1555 */                     }
/* FP:resolve_bound_vars.rs-1556 */                     kind => span_bug!(
/* FP:resolve_bound_vars.rs-1557 */                         use_span,
/* FP:resolve_bound_vars.rs-1558 */                         "did not expect to resolve non-lifetime param to {}",
/* FP:resolve_bound_vars.rs-1559 */                         kind.descr(param_def_id.to_def_id())
/* FP:resolve_bound_vars.rs-1560 */                     ),
/* FP:resolve_bound_vars.rs-1561 */                 };
/* FP:resolve_bound_vars.rs-1562 */                 self.rbv.defs.insert(hir_id.local_id, ResolvedArg::Error(guar));
/* FP:resolve_bound_vars.rs-1563 */             } else {
/* FP:resolve_bound_vars.rs-1564 */                 self.rbv.defs.insert(hir_id.local_id, def);
/* FP:resolve_bound_vars.rs-1565 */             }
/* FP:resolve_bound_vars.rs-1566 */             return;
/* FP:resolve_bound_vars.rs-1567 */         }
/* FP:resolve_bound_vars.rs-1568 */ 
/* FP:resolve_bound_vars.rs-1569 */         // We may fail to resolve higher-ranked ty/const vars that are mentioned by APIT.
/* FP:resolve_bound_vars.rs-1570 */         // AST-based resolution does not care for impl-trait desugaring, which are the
/* FP:resolve_bound_vars.rs-1571 */         // responsibility of lowering. This may create a mismatch between the resolution
/* FP:resolve_bound_vars.rs-1572 */         // AST found (`param_def_id`) which points to HRTB, and what HIR allows.
/* FP:resolve_bound_vars.rs-1573 */         // ```
/* FP:resolve_bound_vars.rs-1574 */         // fn foo(x: impl for<T> Trait<Assoc = impl Trait2<T>>) {}
/* FP:resolve_bound_vars.rs-1575 */         // ```
/* FP:resolve_bound_vars.rs-1576 */         //
/* FP:resolve_bound_vars.rs-1577 */         // In such case, walk back the binders to diagnose it properly.
/* FP:resolve_bound_vars.rs-1578 */         let mut scope = self.scope;
/* FP:resolve_bound_vars.rs-1579 */         loop {
/* FP:resolve_bound_vars.rs-1580 */             match *scope {
/* FP:resolve_bound_vars.rs-1581 */                 Scope::Binder {
/* FP:resolve_bound_vars.rs-1582 */                     where_bound_origin: Some(hir::PredicateOrigin::ImplTrait), ..
/* FP:resolve_bound_vars.rs-1583 */                 } => {
/* FP:resolve_bound_vars.rs-1584 */                     let guar = self.tcx.dcx().emit_err(match self.tcx.def_kind(param_def_id) {
/* FP:resolve_bound_vars.rs-1585 */                         DefKind::TyParam => errors::LateBoundInApit::Type {
/* FP:resolve_bound_vars.rs-1586 */                             span: self.tcx.hir_span(hir_id),
/* FP:resolve_bound_vars.rs-1587 */                             param_span: self.tcx.def_span(param_def_id),
/* FP:resolve_bound_vars.rs-1588 */                         },
/* FP:resolve_bound_vars.rs-1589 */                         DefKind::ConstParam => errors::LateBoundInApit::Const {
/* FP:resolve_bound_vars.rs-1590 */                             span: self.tcx.hir_span(hir_id),
/* FP:resolve_bound_vars.rs-1591 */                             param_span: self.tcx.def_span(param_def_id),
/* FP:resolve_bound_vars.rs-1592 */                         },
/* FP:resolve_bound_vars.rs-1593 */                         kind => {
/* FP:resolve_bound_vars.rs-1594 */                             bug!("unexpected def-kind: {}", kind.descr(param_def_id.to_def_id()))
/* FP:resolve_bound_vars.rs-1595 */                         }
/* FP:resolve_bound_vars.rs-1596 */                     });
/* FP:resolve_bound_vars.rs-1597 */                     self.rbv.defs.insert(hir_id.local_id, ResolvedArg::Error(guar));
/* FP:resolve_bound_vars.rs-1598 */                     return;
/* FP:resolve_bound_vars.rs-1599 */                 }
/* FP:resolve_bound_vars.rs-1600 */                 Scope::Root { .. } => break,
/* FP:resolve_bound_vars.rs-1601 */                 Scope::Binder { s, .. }
/* FP:resolve_bound_vars.rs-1602 */                 | Scope::Body { s, .. }
/* FP:resolve_bound_vars.rs-1603 */                 | Scope::Opaque { s, .. }
/* FP:resolve_bound_vars.rs-1604 */                 | Scope::ObjectLifetimeDefault { s, .. }
/* FP:resolve_bound_vars.rs-1605 */                 | Scope::Supertrait { s, .. }
/* FP:resolve_bound_vars.rs-1606 */                 | Scope::TraitRefBoundary { s, .. }
/* FP:resolve_bound_vars.rs-1607 */                 | Scope::LateBoundary { s, .. } => {
/* FP:resolve_bound_vars.rs-1608 */                     scope = s;
/* FP:resolve_bound_vars.rs-1609 */                 }
/* FP:resolve_bound_vars.rs-1610 */             }
/* FP:resolve_bound_vars.rs-1611 */         }
/* FP:resolve_bound_vars.rs-1612 */ 
/* FP:resolve_bound_vars.rs-1613 */         self.tcx
/* FP:resolve_bound_vars.rs-1614 */             .dcx()
/* FP:resolve_bound_vars.rs-1615 */             .span_bug(self.tcx.hir_span(hir_id), format!("could not resolve {param_def_id:?}"));
/* FP:resolve_bound_vars.rs-1616 */     }
/* FP:resolve_bound_vars.rs-1617 */ 
/* FP:resolve_bound_vars.rs-1618 */     #[instrument(level = "debug", skip(self))]
/* FP:resolve_bound_vars.rs-1619 */     fn visit_segment_args(
/* FP:resolve_bound_vars.rs-1620 */         &mut self,
/* FP:resolve_bound_vars.rs-1621 */         res: Res,
/* FP:resolve_bound_vars.rs-1622 */         depth: usize,
/* FP:resolve_bound_vars.rs-1623 */         generic_args: &'tcx hir::GenericArgs<'tcx>,
/* FP:resolve_bound_vars.rs-1624 */     ) {
/* FP:resolve_bound_vars.rs-1625 */         if let Some((inputs, output)) = generic_args.paren_sugar_inputs_output() {
/* FP:resolve_bound_vars.rs-1626 */             self.visit_fn_like_elision(inputs, Some(output), false);
/* FP:resolve_bound_vars.rs-1627 */             return;
/* FP:resolve_bound_vars.rs-1628 */         }
/* FP:resolve_bound_vars.rs-1629 */ 
/* FP:resolve_bound_vars.rs-1630 */         for arg in generic_args.args {
/* FP:resolve_bound_vars.rs-1631 */             if let hir::GenericArg::Lifetime(lt) = arg {
/* FP:resolve_bound_vars.rs-1632 */                 self.visit_lifetime(lt);
/* FP:resolve_bound_vars.rs-1633 */             }
/* FP:resolve_bound_vars.rs-1634 */         }
/* FP:resolve_bound_vars.rs-1635 */ 
/* FP:resolve_bound_vars.rs-1636 */         // Figure out if this is a type/trait segment,
/* FP:resolve_bound_vars.rs-1637 */         // which requires object lifetime defaults.
/* FP:resolve_bound_vars.rs-1638 */         let type_def_id = match res {
/* FP:resolve_bound_vars.rs-1639 */             Res::Def(DefKind::AssocTy, def_id) if depth == 1 => Some(self.tcx.parent(def_id)),
/* FP:resolve_bound_vars.rs-1640 */             Res::Def(DefKind::Variant, def_id) if depth == 0 => Some(self.tcx.parent(def_id)),
/* FP:resolve_bound_vars.rs-1641 */             Res::Def(
/* FP:resolve_bound_vars.rs-1642 */                 DefKind::Struct
/* FP:resolve_bound_vars.rs-1643 */                 | DefKind::Union
/* FP:resolve_bound_vars.rs-1644 */                 | DefKind::Enum
/* FP:resolve_bound_vars.rs-1645 */                 | DefKind::TyAlias
/* FP:resolve_bound_vars.rs-1646 */                 | DefKind::Trait,
/* FP:resolve_bound_vars.rs-1647 */                 def_id,
/* FP:resolve_bound_vars.rs-1648 */             ) if depth == 0 => Some(def_id),
/* FP:resolve_bound_vars.rs-1649 */             _ => None,
/* FP:resolve_bound_vars.rs-1650 */         };
/* FP:resolve_bound_vars.rs-1651 */ 
/* FP:resolve_bound_vars.rs-1652 */         debug!(?type_def_id);
/* FP:resolve_bound_vars.rs-1653 */ 
/* FP:resolve_bound_vars.rs-1654 */         // Compute a vector of defaults, one for each type parameter,
/* FP:resolve_bound_vars.rs-1655 */         // per the rules given in RFCs 599 and 1156. Example:
/* FP:resolve_bound_vars.rs-1656 */         //
/* FP:resolve_bound_vars.rs-1657 */         // ```rust
/* FP:resolve_bound_vars.rs-1658 */         // struct Foo<'a, T: 'a, U> { }
/* FP:resolve_bound_vars.rs-1659 */         // ```
/* FP:resolve_bound_vars.rs-1660 */         //
/* FP:resolve_bound_vars.rs-1661 */         // If you have `Foo<'x, dyn Bar, dyn Baz>`, we want to default
/* FP:resolve_bound_vars.rs-1662 */         // `dyn Bar` to `dyn Bar + 'x` (because of the `T: 'a` bound)
/* FP:resolve_bound_vars.rs-1663 */         // and `dyn Baz` to `dyn Baz + 'static` (because there is no
/* FP:resolve_bound_vars.rs-1664 */         // such bound).
/* FP:resolve_bound_vars.rs-1665 */         //
/* FP:resolve_bound_vars.rs-1666 */         // Therefore, we would compute `object_lifetime_defaults` to a
/* FP:resolve_bound_vars.rs-1667 */         // vector like `['x, 'static]`. Note that the vector only
/* FP:resolve_bound_vars.rs-1668 */         // includes type parameters.
/* FP:resolve_bound_vars.rs-1669 */         let object_lifetime_defaults = type_def_id.map_or_else(Vec::new, |def_id| {
/* FP:resolve_bound_vars.rs-1670 */             let in_body = {
/* FP:resolve_bound_vars.rs-1671 */                 let mut scope = self.scope;
/* FP:resolve_bound_vars.rs-1672 */                 loop {
/* FP:resolve_bound_vars.rs-1673 */                     match *scope {
/* FP:resolve_bound_vars.rs-1674 */                         Scope::Root { .. } => break false,
/* FP:resolve_bound_vars.rs-1675 */ 
/* FP:resolve_bound_vars.rs-1676 */                         Scope::Body { .. } => break true,
/* FP:resolve_bound_vars.rs-1677 */ 
/* FP:resolve_bound_vars.rs-1678 */                         Scope::Binder { s, .. }
/* FP:resolve_bound_vars.rs-1679 */                         | Scope::ObjectLifetimeDefault { s, .. }
/* FP:resolve_bound_vars.rs-1680 */                         | Scope::Opaque { s, .. }
/* FP:resolve_bound_vars.rs-1681 */                         | Scope::Supertrait { s, .. }
/* FP:resolve_bound_vars.rs-1682 */                         | Scope::TraitRefBoundary { s, .. }
/* FP:resolve_bound_vars.rs-1683 */                         | Scope::LateBoundary { s, .. } => {
/* FP:resolve_bound_vars.rs-1684 */                             scope = s;
/* FP:resolve_bound_vars.rs-1685 */                         }
/* FP:resolve_bound_vars.rs-1686 */                     }
/* FP:resolve_bound_vars.rs-1687 */                 }
/* FP:resolve_bound_vars.rs-1688 */             };
/* FP:resolve_bound_vars.rs-1689 */ 
/* FP:resolve_bound_vars.rs-1690 */             let rbv = &self.rbv;
/* FP:resolve_bound_vars.rs-1691 */             let generics = self.tcx.generics_of(def_id);
/* FP:resolve_bound_vars.rs-1692 */ 
/* FP:resolve_bound_vars.rs-1693 */             // `type_def_id` points to an item, so there is nothing to inherit generics from.
/* FP:resolve_bound_vars.rs-1694 */             debug_assert_eq!(generics.parent_count, 0);
/* FP:resolve_bound_vars.rs-1695 */ 
/* FP:resolve_bound_vars.rs-1696 */             let set_to_region = |set: ObjectLifetimeDefault| match set {
/* FP:resolve_bound_vars.rs-1697 */                 ObjectLifetimeDefault::Empty => {
/* FP:resolve_bound_vars.rs-1698 */                     if in_body {
/* FP:resolve_bound_vars.rs-1699 */                         None
/* FP:resolve_bound_vars.rs-1700 */                     } else {
/* FP:resolve_bound_vars.rs-1701 */                         Some(ResolvedArg::StaticLifetime)
/* FP:resolve_bound_vars.rs-1702 */                     }
/* FP:resolve_bound_vars.rs-1703 */                 }
/* FP:resolve_bound_vars.rs-1704 */                 ObjectLifetimeDefault::Static => Some(ResolvedArg::StaticLifetime),
/* FP:resolve_bound_vars.rs-1705 */                 ObjectLifetimeDefault::Param(param_def_id) => {
/* FP:resolve_bound_vars.rs-1706 */                     // This index can be used with `generic_args` since `parent_count == 0`.
/* FP:resolve_bound_vars.rs-1707 */                     let index = generics.param_def_id_to_index[&param_def_id] as usize;
/* FP:resolve_bound_vars.rs-1708 */                     generic_args.args.get(index).and_then(|arg| match arg {
/* FP:resolve_bound_vars.rs-1709 */                         GenericArg::Lifetime(lt) => rbv.defs.get(&lt.hir_id.local_id).copied(),
/* FP:resolve_bound_vars.rs-1710 */                         _ => None,
/* FP:resolve_bound_vars.rs-1711 */                     })
/* FP:resolve_bound_vars.rs-1712 */                 }
/* FP:resolve_bound_vars.rs-1713 */                 ObjectLifetimeDefault::Ambiguous => None,
/* FP:resolve_bound_vars.rs-1714 */             };
/* FP:resolve_bound_vars.rs-1715 */             generics
/* FP:resolve_bound_vars.rs-1716 */                 .own_params
/* FP:resolve_bound_vars.rs-1717 */                 .iter()
/* FP:resolve_bound_vars.rs-1718 */                 .filter_map(|param| {
/* FP:resolve_bound_vars.rs-1719 */                     match self.tcx.def_kind(param.def_id) {
/* FP:resolve_bound_vars.rs-1720 */                         // Generic consts don't impose any constraints.
/* FP:resolve_bound_vars.rs-1721 */                         //
/* FP:resolve_bound_vars.rs-1722 */                         // We still store a dummy value here to allow generic parameters
/* FP:resolve_bound_vars.rs-1723 */                         // in an arbitrary order.
/* FP:resolve_bound_vars.rs-1724 */                         DefKind::ConstParam => Some(ObjectLifetimeDefault::Empty),
/* FP:resolve_bound_vars.rs-1725 */                         DefKind::TyParam => Some(self.tcx.object_lifetime_default(param.def_id)),
/* FP:resolve_bound_vars.rs-1726 */                         // We may also get a `Trait` or `TraitAlias` because of how generics `Self` parameter
/* FP:resolve_bound_vars.rs-1727 */                         // works. Ignore it because it can't have a meaningful lifetime default.
/* FP:resolve_bound_vars.rs-1728 */                         DefKind::LifetimeParam | DefKind::Trait | DefKind::TraitAlias => None,
/* FP:resolve_bound_vars.rs-1729 */                         dk => bug!("unexpected def_kind {:?}", dk),
/* FP:resolve_bound_vars.rs-1730 */                     }
/* FP:resolve_bound_vars.rs-1731 */                 })
/* FP:resolve_bound_vars.rs-1732 */                 .map(set_to_region)
/* FP:resolve_bound_vars.rs-1733 */                 .collect()
/* FP:resolve_bound_vars.rs-1734 */         });
/* FP:resolve_bound_vars.rs-1735 */ 
/* FP:resolve_bound_vars.rs-1736 */         debug!(?object_lifetime_defaults);
/* FP:resolve_bound_vars.rs-1737 */ 
/* FP:resolve_bound_vars.rs-1738 */         let mut i = 0;
/* FP:resolve_bound_vars.rs-1739 */         for arg in generic_args.args {
/* FP:resolve_bound_vars.rs-1740 */             match arg {
/* FP:resolve_bound_vars.rs-1741 */                 GenericArg::Lifetime(_) => {}
/* FP:resolve_bound_vars.rs-1742 */                 GenericArg::Type(ty) => {
/* FP:resolve_bound_vars.rs-1743 */                     if let Some(&lt) = object_lifetime_defaults.get(i) {
/* FP:resolve_bound_vars.rs-1744 */                         let scope = Scope::ObjectLifetimeDefault { lifetime: lt, s: self.scope };
/* FP:resolve_bound_vars.rs-1745 */                         self.with(scope, |this| this.visit_ty(ty));
/* FP:resolve_bound_vars.rs-1746 */                     } else {
/* FP:resolve_bound_vars.rs-1747 */                         self.visit_ty(ty);
/* FP:resolve_bound_vars.rs-1748 */                     }
/* FP:resolve_bound_vars.rs-1749 */                     i += 1;
/* FP:resolve_bound_vars.rs-1750 */                 }
/* FP:resolve_bound_vars.rs-1751 */                 GenericArg::Const(ct) => {
/* FP:resolve_bound_vars.rs-1752 */                     self.visit_const_arg(ct);
/* FP:resolve_bound_vars.rs-1753 */                     i += 1;
/* FP:resolve_bound_vars.rs-1754 */                 }
/* FP:resolve_bound_vars.rs-1755 */                 GenericArg::Infer(inf) => {
/* FP:resolve_bound_vars.rs-1756 */                     self.visit_id(inf.hir_id);
/* FP:resolve_bound_vars.rs-1757 */                     i += 1;
/* FP:resolve_bound_vars.rs-1758 */                 }
/* FP:resolve_bound_vars.rs-1759 */             }
/* FP:resolve_bound_vars.rs-1760 */         }
/* FP:resolve_bound_vars.rs-1761 */ 
/* FP:resolve_bound_vars.rs-1762 */         // Hack: When resolving the type `XX` in an assoc ty binding like
/* FP:resolve_bound_vars.rs-1763 */         // `dyn Foo<'b, Item = XX>`, the current object-lifetime default
/* FP:resolve_bound_vars.rs-1764 */         // would be to examine the trait `Foo` to check whether it has
/* FP:resolve_bound_vars.rs-1765 */         // a lifetime bound declared on `Item`. e.g., if `Foo` is
/* FP:resolve_bound_vars.rs-1766 */         // declared like so, then the default object lifetime bound in
/* FP:resolve_bound_vars.rs-1767 */         // `XX` should be `'b`:
/* FP:resolve_bound_vars.rs-1768 */         //
/* FP:resolve_bound_vars.rs-1769 */         // ```rust
/* FP:resolve_bound_vars.rs-1770 */         // trait Foo<'a> {
/* FP:resolve_bound_vars.rs-1771 */         //   type Item: 'a;
/* FP:resolve_bound_vars.rs-1772 */         // }
/* FP:resolve_bound_vars.rs-1773 */         // ```
/* FP:resolve_bound_vars.rs-1774 */         //
/* FP:resolve_bound_vars.rs-1775 */         // but if we just have `type Item;`, then it would be
/* FP:resolve_bound_vars.rs-1776 */         // `'static`. However, we don't get all of this logic correct.
/* FP:resolve_bound_vars.rs-1777 */         //
/* FP:resolve_bound_vars.rs-1778 */         // Instead, we do something hacky: if there are no lifetime parameters
/* FP:resolve_bound_vars.rs-1779 */         // to the trait, then we simply use a default object lifetime
/* FP:resolve_bound_vars.rs-1780 */         // bound of `'static`, because there is no other possibility. On the other hand,
/* FP:resolve_bound_vars.rs-1781 */         // if there ARE lifetime parameters, then we require the user to give an
/* FP:resolve_bound_vars.rs-1782 */         // explicit bound for now.
/* FP:resolve_bound_vars.rs-1783 */         //
/* FP:resolve_bound_vars.rs-1784 */         // This is intended to leave room for us to implement the
/* FP:resolve_bound_vars.rs-1785 */         // correct behavior in the future.
/* FP:resolve_bound_vars.rs-1786 */         let has_lifetime_parameter =
/* FP:resolve_bound_vars.rs-1787 */             generic_args.args.iter().any(|arg| matches!(arg, GenericArg::Lifetime(_)));
/* FP:resolve_bound_vars.rs-1788 */ 
/* FP:resolve_bound_vars.rs-1789 */         // Resolve lifetimes found in the bindings, so either in the type `XX` in `Item = XX` or
/* FP:resolve_bound_vars.rs-1790 */         // in the trait ref `YY<...>` in `Item: YY<...>`.
/* FP:resolve_bound_vars.rs-1791 */         for constraint in generic_args.constraints {
/* FP:resolve_bound_vars.rs-1792 */             let scope = Scope::ObjectLifetimeDefault {
/* FP:resolve_bound_vars.rs-1793 */                 lifetime: if has_lifetime_parameter {
/* FP:resolve_bound_vars.rs-1794 */                     None
/* FP:resolve_bound_vars.rs-1795 */                 } else {
/* FP:resolve_bound_vars.rs-1796 */                     Some(ResolvedArg::StaticLifetime)
/* FP:resolve_bound_vars.rs-1797 */                 },
/* FP:resolve_bound_vars.rs-1798 */                 s: self.scope,
/* FP:resolve_bound_vars.rs-1799 */             };
/* FP:resolve_bound_vars.rs-1800 */             // If the args are parenthesized, then this must be `feature(return_type_notation)`.
/* FP:resolve_bound_vars.rs-1801 */             // In that case, introduce a binder over all of the function's early and late bound vars.
/* FP:resolve_bound_vars.rs-1802 */             //
/* FP:resolve_bound_vars.rs-1803 */             // For example, given
/* FP:resolve_bound_vars.rs-1804 */             // ```
/* FP:resolve_bound_vars.rs-1805 */             // trait Foo {
/* FP:resolve_bound_vars.rs-1806 */             //     async fn x<'r, T>();
/* FP:resolve_bound_vars.rs-1807 */             // }
/* FP:resolve_bound_vars.rs-1808 */             // ```
/* FP:resolve_bound_vars.rs-1809 */             // and a bound that looks like:
/* FP:resolve_bound_vars.rs-1810 */             //    `for<'a> T::Trait<'a, x(..): for<'b> Other<'b>>`
/* FP:resolve_bound_vars.rs-1811 */             // this is going to expand to something like:
/* FP:resolve_bound_vars.rs-1812 */             //    `for<'a> for<'r> <T as Trait<'a>>::x::<'r, T>::{opaque#0}: for<'b> Other<'b>`.
/* FP:resolve_bound_vars.rs-1813 */             if constraint.gen_args.parenthesized == hir::GenericArgsParentheses::ReturnTypeNotation
/* FP:resolve_bound_vars.rs-1814 */             {
/* FP:resolve_bound_vars.rs-1815 */                 let bound_vars = if let Some(type_def_id) = type_def_id
/* FP:resolve_bound_vars.rs-1816 */                     && self.tcx.def_kind(type_def_id) == DefKind::Trait
/* FP:resolve_bound_vars.rs-1817 */                     && let Some((mut bound_vars, assoc_fn)) = BoundVarContext::supertrait_hrtb_vars(
/* FP:resolve_bound_vars.rs-1818 */                         self.tcx,
/* FP:resolve_bound_vars.rs-1819 */                         type_def_id,
/* FP:resolve_bound_vars.rs-1820 */                         constraint.ident,
/* FP:resolve_bound_vars.rs-1821 */                         ty::AssocTag::Fn,
/* FP:resolve_bound_vars.rs-1822 */                     ) {
/* FP:resolve_bound_vars.rs-1823 */                     bound_vars.extend(
/* FP:resolve_bound_vars.rs-1824 */                         self.tcx
/* FP:resolve_bound_vars.rs-1825 */                             .generics_of(assoc_fn.def_id)
/* FP:resolve_bound_vars.rs-1826 */                             .own_params
/* FP:resolve_bound_vars.rs-1827 */                             .iter()
/* FP:resolve_bound_vars.rs-1828 */                             .map(|param| generic_param_def_as_bound_arg(param)),
/* FP:resolve_bound_vars.rs-1829 */                     );
/* FP:resolve_bound_vars.rs-1830 */                     bound_vars.extend(
/* FP:resolve_bound_vars.rs-1831 */                         self.tcx.fn_sig(assoc_fn.def_id).instantiate_identity().bound_vars(),
/* FP:resolve_bound_vars.rs-1832 */                     );
/* FP:resolve_bound_vars.rs-1833 */                     bound_vars
/* FP:resolve_bound_vars.rs-1834 */                 } else {
/* FP:resolve_bound_vars.rs-1835 */                     self.tcx
/* FP:resolve_bound_vars.rs-1836 */                         .dcx()
/* FP:resolve_bound_vars.rs-1837 */                         .span_delayed_bug(constraint.ident.span, "bad return type notation here");
/* FP:resolve_bound_vars.rs-1838 */                     vec![]
/* FP:resolve_bound_vars.rs-1839 */                 };
/* FP:resolve_bound_vars.rs-1840 */                 self.with(scope, |this| {
/* FP:resolve_bound_vars.rs-1841 */                     let scope = Scope::Supertrait { bound_vars, s: this.scope };
/* FP:resolve_bound_vars.rs-1842 */                     this.with(scope, |this| {
/* FP:resolve_bound_vars.rs-1843 */                         let (bound_vars, _) = this.poly_trait_ref_binder_info();
/* FP:resolve_bound_vars.rs-1844 */                         this.record_late_bound_vars(constraint.hir_id, bound_vars);
/* FP:resolve_bound_vars.rs-1845 */                         this.visit_assoc_item_constraint(constraint)
/* FP:resolve_bound_vars.rs-1846 */                     });
/* FP:resolve_bound_vars.rs-1847 */                 });
/* FP:resolve_bound_vars.rs-1848 */             } else if let Some(type_def_id) = type_def_id {
/* FP:resolve_bound_vars.rs-1849 */                 let bound_vars = BoundVarContext::supertrait_hrtb_vars(
/* FP:resolve_bound_vars.rs-1850 */                     self.tcx,
/* FP:resolve_bound_vars.rs-1851 */                     type_def_id,
/* FP:resolve_bound_vars.rs-1852 */                     constraint.ident,
/* FP:resolve_bound_vars.rs-1853 */                     ty::AssocTag::Type,
/* FP:resolve_bound_vars.rs-1854 */                 )
/* FP:resolve_bound_vars.rs-1855 */                 .map(|(bound_vars, _)| bound_vars);
/* FP:resolve_bound_vars.rs-1856 */                 self.with(scope, |this| {
/* FP:resolve_bound_vars.rs-1857 */                     let scope = Scope::Supertrait {
/* FP:resolve_bound_vars.rs-1858 */                         bound_vars: bound_vars.unwrap_or_default(),
/* FP:resolve_bound_vars.rs-1859 */                         s: this.scope,
/* FP:resolve_bound_vars.rs-1860 */                     };
/* FP:resolve_bound_vars.rs-1861 */                     this.with(scope, |this| this.visit_assoc_item_constraint(constraint));
/* FP:resolve_bound_vars.rs-1862 */                 });
/* FP:resolve_bound_vars.rs-1863 */             } else {
/* FP:resolve_bound_vars.rs-1864 */                 self.with(scope, |this| this.visit_assoc_item_constraint(constraint));
/* FP:resolve_bound_vars.rs-1865 */             }
/* FP:resolve_bound_vars.rs-1866 */         }
/* FP:resolve_bound_vars.rs-1867 */     }
/* FP:resolve_bound_vars.rs-1868 */ 
/* FP:resolve_bound_vars.rs-1869 */     /// Returns all the late-bound vars that come into scope from supertrait HRTBs, based on the
/* FP:resolve_bound_vars.rs-1870 */     /// associated type name and starting trait.
/* FP:resolve_bound_vars.rs-1871 */     /// For example, imagine we have
/* FP:resolve_bound_vars.rs-1872 */     /// ```ignore (illustrative)
/* FP:resolve_bound_vars.rs-1873 */     /// trait Foo<'a, 'b> {
/* FP:resolve_bound_vars.rs-1874 */     ///   type As;
/* FP:resolve_bound_vars.rs-1875 */     /// }
/* FP:resolve_bound_vars.rs-1876 */     /// trait Bar<'b>: for<'a> Foo<'a, 'b> {}
/* FP:resolve_bound_vars.rs-1877 */     /// trait Bar: for<'b> Bar<'b> {}
/* FP:resolve_bound_vars.rs-1878 */     /// ```
/* FP:resolve_bound_vars.rs-1879 */     /// In this case, if we wanted to the supertrait HRTB lifetimes for `As` on
/* FP:resolve_bound_vars.rs-1880 */     /// the starting trait `Bar`, we would return `Some(['b, 'a])`.
/* FP:resolve_bound_vars.rs-1881 */     fn supertrait_hrtb_vars(
/* FP:resolve_bound_vars.rs-1882 */         tcx: TyCtxt<'tcx>,
/* FP:resolve_bound_vars.rs-1883 */         def_id: DefId,
/* FP:resolve_bound_vars.rs-1884 */         assoc_ident: Ident,
/* FP:resolve_bound_vars.rs-1885 */         assoc_tag: ty::AssocTag,
/* FP:resolve_bound_vars.rs-1886 */     ) -> Option<(Vec<ty::BoundVariableKind>, &'tcx ty::AssocItem)> {
/* FP:resolve_bound_vars.rs-1887 */         let trait_defines_associated_item_named = |trait_def_id: DefId| {
/* FP:resolve_bound_vars.rs-1888 */             tcx.associated_items(trait_def_id).find_by_ident_and_kind(
/* FP:resolve_bound_vars.rs-1889 */                 tcx,
/* FP:resolve_bound_vars.rs-1890 */                 assoc_ident,
/* FP:resolve_bound_vars.rs-1891 */                 assoc_tag,
/* FP:resolve_bound_vars.rs-1892 */                 trait_def_id,
/* FP:resolve_bound_vars.rs-1893 */             )
/* FP:resolve_bound_vars.rs-1894 */         };
/* FP:resolve_bound_vars.rs-1895 */ 
/* FP:resolve_bound_vars.rs-1896 */         use smallvec::{SmallVec, smallvec};
/* FP:resolve_bound_vars.rs-1897 */         let mut stack: SmallVec<[(DefId, SmallVec<[ty::BoundVariableKind; 8]>); 8]> =
/* FP:resolve_bound_vars.rs-1898 */             smallvec![(def_id, smallvec![])];
/* FP:resolve_bound_vars.rs-1899 */         let mut visited: FxHashSet<DefId> = FxHashSet::default();
/* FP:resolve_bound_vars.rs-1900 */         loop {
/* FP:resolve_bound_vars.rs-1901 */             let Some((def_id, bound_vars)) = stack.pop() else {
/* FP:resolve_bound_vars.rs-1902 */                 break None;
/* FP:resolve_bound_vars.rs-1903 */             };
/* FP:resolve_bound_vars.rs-1904 */             // See issue #83753. If someone writes an associated type on a non-trait, just treat it
/* FP:resolve_bound_vars.rs-1905 */             // as there being no supertrait HRTBs.
/* FP:resolve_bound_vars.rs-1906 */             match tcx.def_kind(def_id) {
/* FP:resolve_bound_vars.rs-1907 */                 DefKind::Trait | DefKind::TraitAlias | DefKind::Impl { .. } => {}
/* FP:resolve_bound_vars.rs-1908 */                 _ => break None,
/* FP:resolve_bound_vars.rs-1909 */             }
/* FP:resolve_bound_vars.rs-1910 */ 
/* FP:resolve_bound_vars.rs-1911 */             if let Some(assoc_item) = trait_defines_associated_item_named(def_id) {
/* FP:resolve_bound_vars.rs-1912 */                 break Some((bound_vars.into_iter().collect(), assoc_item));
/* FP:resolve_bound_vars.rs-1913 */             }
/* FP:resolve_bound_vars.rs-1914 */             let predicates = tcx.explicit_supertraits_containing_assoc_item((def_id, assoc_ident));
/* FP:resolve_bound_vars.rs-1915 */             let obligations = predicates.iter_identity_copied().filter_map(|(pred, _)| {
/* FP:resolve_bound_vars.rs-1916 */                 let bound_predicate = pred.kind();
/* FP:resolve_bound_vars.rs-1917 */                 match bound_predicate.skip_binder() {
/* FP:resolve_bound_vars.rs-1918 */                     ty::ClauseKind::Trait(data) => {
/* FP:resolve_bound_vars.rs-1919 */                         // The order here needs to match what we would get from
/* FP:resolve_bound_vars.rs-1920 */                         // `crate::rustc_middle::ty::predicate::Clause::instantiate_supertrait`
/* FP:resolve_bound_vars.rs-1921 */                         let pred_bound_vars = bound_predicate.bound_vars();
/* FP:resolve_bound_vars.rs-1922 */                         let mut all_bound_vars = bound_vars.clone();
/* FP:resolve_bound_vars.rs-1923 */                         all_bound_vars.extend(pred_bound_vars.iter());
/* FP:resolve_bound_vars.rs-1924 */                         let super_def_id = data.trait_ref.def_id;
/* FP:resolve_bound_vars.rs-1925 */                         Some((super_def_id, all_bound_vars))
/* FP:resolve_bound_vars.rs-1926 */                     }
/* FP:resolve_bound_vars.rs-1927 */                     _ => None,
/* FP:resolve_bound_vars.rs-1928 */                 }
/* FP:resolve_bound_vars.rs-1929 */             });
/* FP:resolve_bound_vars.rs-1930 */ 
/* FP:resolve_bound_vars.rs-1931 */             let obligations = obligations.filter(|o| visited.insert(o.0));
/* FP:resolve_bound_vars.rs-1932 */             stack.extend(obligations);
/* FP:resolve_bound_vars.rs-1933 */         }
/* FP:resolve_bound_vars.rs-1934 */     }
/* FP:resolve_bound_vars.rs-1935 */ 
/* FP:resolve_bound_vars.rs-1936 */     #[instrument(level = "debug", skip(self))]
/* FP:resolve_bound_vars.rs-1937 */     fn visit_fn_like_elision(
/* FP:resolve_bound_vars.rs-1938 */         &mut self,
/* FP:resolve_bound_vars.rs-1939 */         inputs: &'tcx [hir::Ty<'tcx>],
/* FP:resolve_bound_vars.rs-1940 */         output: Option<&'tcx hir::Ty<'tcx>>,
/* FP:resolve_bound_vars.rs-1941 */         in_closure: bool,
/* FP:resolve_bound_vars.rs-1942 */     ) {
/* FP:resolve_bound_vars.rs-1943 */         self.with(
/* FP:resolve_bound_vars.rs-1944 */             Scope::ObjectLifetimeDefault {
/* FP:resolve_bound_vars.rs-1945 */                 lifetime: Some(ResolvedArg::StaticLifetime),
/* FP:resolve_bound_vars.rs-1946 */                 s: self.scope,
/* FP:resolve_bound_vars.rs-1947 */             },
/* FP:resolve_bound_vars.rs-1948 */             |this| {
/* FP:resolve_bound_vars.rs-1949 */                 for input in inputs {
/* FP:resolve_bound_vars.rs-1950 */                     this.visit_ty_unambig(input);
/* FP:resolve_bound_vars.rs-1951 */                 }
/* FP:resolve_bound_vars.rs-1952 */                 if !in_closure && let Some(output) = output {
/* FP:resolve_bound_vars.rs-1953 */                     this.visit_ty_unambig(output);
/* FP:resolve_bound_vars.rs-1954 */                 }
/* FP:resolve_bound_vars.rs-1955 */             },
/* FP:resolve_bound_vars.rs-1956 */         );
/* FP:resolve_bound_vars.rs-1957 */         if in_closure && let Some(output) = output {
/* FP:resolve_bound_vars.rs-1958 */             self.visit_ty_unambig(output);
/* FP:resolve_bound_vars.rs-1959 */         }
/* FP:resolve_bound_vars.rs-1960 */     }
/* FP:resolve_bound_vars.rs-1961 */ 
/* FP:resolve_bound_vars.rs-1962 */     #[instrument(level = "debug", skip(self))]
/* FP:resolve_bound_vars.rs-1963 */     fn resolve_object_lifetime_default(&mut self, lifetime_ref: &'tcx hir::Lifetime) {
/* FP:resolve_bound_vars.rs-1964 */         let mut late_depth = 0;
/* FP:resolve_bound_vars.rs-1965 */         let mut scope = self.scope;
/* FP:resolve_bound_vars.rs-1966 */         let mut opaque_capture_scopes = vec![];
/* FP:resolve_bound_vars.rs-1967 */         let mut lifetime = loop {
/* FP:resolve_bound_vars.rs-1968 */             match *scope {
/* FP:resolve_bound_vars.rs-1969 */                 Scope::Binder { s, scope_type, .. } => {
/* FP:resolve_bound_vars.rs-1970 */                     match scope_type {
/* FP:resolve_bound_vars.rs-1971 */                         BinderScopeType::Normal => late_depth += 1,
/* FP:resolve_bound_vars.rs-1972 */                         BinderScopeType::Concatenating => {}
/* FP:resolve_bound_vars.rs-1973 */                     }
/* FP:resolve_bound_vars.rs-1974 */                     scope = s;
/* FP:resolve_bound_vars.rs-1975 */                 }
/* FP:resolve_bound_vars.rs-1976 */ 
/* FP:resolve_bound_vars.rs-1977 */                 Scope::Root { .. } => break ResolvedArg::StaticLifetime,
/* FP:resolve_bound_vars.rs-1978 */ 
/* FP:resolve_bound_vars.rs-1979 */                 Scope::Body { .. } | Scope::ObjectLifetimeDefault { lifetime: None, .. } => return,
/* FP:resolve_bound_vars.rs-1980 */ 
/* FP:resolve_bound_vars.rs-1981 */                 Scope::ObjectLifetimeDefault { lifetime: Some(l), .. } => {
/* FP:resolve_bound_vars.rs-1982 */                     break l.shifted(late_depth);
/* FP:resolve_bound_vars.rs-1983 */                 }
/* FP:resolve_bound_vars.rs-1984 */ 
/* FP:resolve_bound_vars.rs-1985 */                 Scope::Opaque { captures, def_id, s } => {
/* FP:resolve_bound_vars.rs-1986 */                     opaque_capture_scopes.push((def_id, captures));
/* FP:resolve_bound_vars.rs-1987 */                     late_depth = 0;
/* FP:resolve_bound_vars.rs-1988 */                     scope = s;
/* FP:resolve_bound_vars.rs-1989 */                 }
/* FP:resolve_bound_vars.rs-1990 */ 
/* FP:resolve_bound_vars.rs-1991 */                 Scope::Supertrait { s, .. }
/* FP:resolve_bound_vars.rs-1992 */                 | Scope::TraitRefBoundary { s, .. }
/* FP:resolve_bound_vars.rs-1993 */                 | Scope::LateBoundary { s, .. } => {
/* FP:resolve_bound_vars.rs-1994 */                     scope = s;
/* FP:resolve_bound_vars.rs-1995 */                 }
/* FP:resolve_bound_vars.rs-1996 */             }
/* FP:resolve_bound_vars.rs-1997 */         };
/* FP:resolve_bound_vars.rs-1998 */ 
/* FP:resolve_bound_vars.rs-1999 */         lifetime = self.remap_opaque_captures(&opaque_capture_scopes, lifetime, lifetime_ref.ident);
/* FP:resolve_bound_vars.rs-2000 */ 
/* FP:resolve_bound_vars.rs-2001 */         self.insert_lifetime(lifetime_ref, lifetime);
/* FP:resolve_bound_vars.rs-2002 */     }
/* FP:resolve_bound_vars.rs-2003 */ 
/* FP:resolve_bound_vars.rs-2004 */     #[instrument(level = "debug", skip(self))]
/* FP:resolve_bound_vars.rs-2005 */     fn insert_lifetime(&mut self, lifetime_ref: &'tcx hir::Lifetime, def: ResolvedArg) {
/* FP:resolve_bound_vars.rs-2006 */         debug!(span = ?lifetime_ref.ident.span);
/* FP:resolve_bound_vars.rs-2007 */         self.rbv.defs.insert(lifetime_ref.hir_id.local_id, def);
/* FP:resolve_bound_vars.rs-2008 */     }
/* FP:resolve_bound_vars.rs-2009 */ 
/* FP:resolve_bound_vars.rs-2010 */     // When we have a return type notation type in a where clause, like
/* FP:resolve_bound_vars.rs-2011 */     // `where <T as Trait>::method(..): Send`, we need to introduce new bound
/* FP:resolve_bound_vars.rs-2012 */     // vars to the existing where clause's binder, to represent the lifetimes
/* FP:resolve_bound_vars.rs-2013 */     // elided by the return-type-notation syntax.
/* FP:resolve_bound_vars.rs-2014 */     //
/* FP:resolve_bound_vars.rs-2015 */     // For example, given
/* FP:resolve_bound_vars.rs-2016 */     // ```
/* FP:resolve_bound_vars.rs-2017 */     // trait Foo {
/* FP:resolve_bound_vars.rs-2018 */     //     async fn x<'r>();
/* FP:resolve_bound_vars.rs-2019 */     // }
/* FP:resolve_bound_vars.rs-2020 */     // ```
/* FP:resolve_bound_vars.rs-2021 */     // and a bound that looks like:
/* FP:resolve_bound_vars.rs-2022 */     //    `for<'a, 'b> <T as Trait<'a>>::x(): Other<'b>`
/* FP:resolve_bound_vars.rs-2023 */     // this is going to expand to something like:
/* FP:resolve_bound_vars.rs-2024 */     //    `for<'a, 'b, 'r> <T as Trait<'a>>::x::<'r, T>::{opaque#0}: Other<'b>`.
/* FP:resolve_bound_vars.rs-2025 */     //
/* FP:resolve_bound_vars.rs-2026 */     // We handle this similarly for associated-type-bound style return-type-notation
/* FP:resolve_bound_vars.rs-2027 */     // in `visit_segment_args`.
/* FP:resolve_bound_vars.rs-2028 */     fn try_append_return_type_notation_params(
/* FP:resolve_bound_vars.rs-2029 */         &mut self,
/* FP:resolve_bound_vars.rs-2030 */         hir_id: HirId,
/* FP:resolve_bound_vars.rs-2031 */         hir_ty: &'tcx hir::Ty<'tcx>,
/* FP:resolve_bound_vars.rs-2032 */     ) {
/* FP:resolve_bound_vars.rs-2033 */         let hir::TyKind::Path(qpath) = hir_ty.kind else {
/* FP:resolve_bound_vars.rs-2034 */             // We only care about path types here. All other self types
/* FP:resolve_bound_vars.rs-2035 */             // (including nesting the RTN type in another type) don't do
/* FP:resolve_bound_vars.rs-2036 */             // anything.
/* FP:resolve_bound_vars.rs-2037 */             return;
/* FP:resolve_bound_vars.rs-2038 */         };
/* FP:resolve_bound_vars.rs-2039 */ 
/* FP:resolve_bound_vars.rs-2040 */         let (mut bound_vars, item_def_id, item_segment) = match qpath {
/* FP:resolve_bound_vars.rs-2041 */             // If we have a fully qualified method, then we don't need to do any special lookup.
/* FP:resolve_bound_vars.rs-2042 */             hir::QPath::Resolved(_, path)
/* FP:resolve_bound_vars.rs-2043 */                 if let [.., item_segment] = &path.segments[..]
/* FP:resolve_bound_vars.rs-2044 */                     && item_segment.args.is_some_and(|args| {
/* FP:resolve_bound_vars.rs-2045 */                         matches!(
/* FP:resolve_bound_vars.rs-2046 */                             args.parenthesized,
/* FP:resolve_bound_vars.rs-2047 */                             hir::GenericArgsParentheses::ReturnTypeNotation
/* FP:resolve_bound_vars.rs-2048 */                         )
/* FP:resolve_bound_vars.rs-2049 */                     }) =>
/* FP:resolve_bound_vars.rs-2050 */             {
/* FP:resolve_bound_vars.rs-2051 */                 match path.res {
/* FP:resolve_bound_vars.rs-2052 */                     Res::Err => return,
/* FP:resolve_bound_vars.rs-2053 */                     Res::Def(DefKind::AssocFn, item_def_id) => (vec![], item_def_id, item_segment),
/* FP:resolve_bound_vars.rs-2054 */                     _ => bug!("only expected method resolution for fully qualified RTN"),
/* FP:resolve_bound_vars.rs-2055 */                 }
/* FP:resolve_bound_vars.rs-2056 */             }
/* FP:resolve_bound_vars.rs-2057 */ 
/* FP:resolve_bound_vars.rs-2058 */             // If we have a type-dependent path, then we do need to do some lookup.
/* FP:resolve_bound_vars.rs-2059 */             hir::QPath::TypeRelative(qself, item_segment)
/* FP:resolve_bound_vars.rs-2060 */                 if item_segment.args.is_some_and(|args| {
/* FP:resolve_bound_vars.rs-2061 */                     matches!(args.parenthesized, hir::GenericArgsParentheses::ReturnTypeNotation)
/* FP:resolve_bound_vars.rs-2062 */                 }) =>
/* FP:resolve_bound_vars.rs-2063 */             {
/* FP:resolve_bound_vars.rs-2064 */                 // First, ignore a qself that isn't a type or `Self` param. Those are the
/* FP:resolve_bound_vars.rs-2065 */                 // only ones that support `T::Assoc` anyways in HIR lowering.
/* FP:resolve_bound_vars.rs-2066 */                 let hir::TyKind::Path(hir::QPath::Resolved(None, path)) = qself.kind else {
/* FP:resolve_bound_vars.rs-2067 */                     return;
/* FP:resolve_bound_vars.rs-2068 */                 };
/* FP:resolve_bound_vars.rs-2069 */                 match path.res {
/* FP:resolve_bound_vars.rs-2070 */                     Res::Def(DefKind::TyParam, _) | Res::SelfTyParam { trait_: _ } => {
/* FP:resolve_bound_vars.rs-2071 */                         let mut bounds =
/* FP:resolve_bound_vars.rs-2072 */                             self.for_each_trait_bound_on_res(path.res).filter_map(|trait_def_id| {
/* FP:resolve_bound_vars.rs-2073 */                                 BoundVarContext::supertrait_hrtb_vars(
/* FP:resolve_bound_vars.rs-2074 */                                     self.tcx,
/* FP:resolve_bound_vars.rs-2075 */                                     trait_def_id,
/* FP:resolve_bound_vars.rs-2076 */                                     item_segment.ident,
/* FP:resolve_bound_vars.rs-2077 */                                     ty::AssocTag::Fn,
/* FP:resolve_bound_vars.rs-2078 */                                 )
/* FP:resolve_bound_vars.rs-2079 */                             });
/* FP:resolve_bound_vars.rs-2080 */ 
/* FP:resolve_bound_vars.rs-2081 */                         let Some((bound_vars, assoc_item)) = bounds.next() else {
/* FP:resolve_bound_vars.rs-2082 */                             // This will error in HIR lowering.
/* FP:resolve_bound_vars.rs-2083 */                             self.tcx
/* FP:resolve_bound_vars.rs-2084 */                                 .dcx()
/* FP:resolve_bound_vars.rs-2085 */                                 .span_delayed_bug(path.span, "no resolution for RTN path");
/* FP:resolve_bound_vars.rs-2086 */                             return;
/* FP:resolve_bound_vars.rs-2087 */                         };
/* FP:resolve_bound_vars.rs-2088 */ 
/* FP:resolve_bound_vars.rs-2089 */                         // Don't bail if we have identical bounds, which may be collected from
/* FP:resolve_bound_vars.rs-2090 */                         // something like `T: Bound + Bound`, or via elaborating supertraits.
/* FP:resolve_bound_vars.rs-2091 */                         for (second_vars, second_assoc_item) in bounds {
/* FP:resolve_bound_vars.rs-2092 */                             if second_vars != bound_vars || second_assoc_item != assoc_item {
/* FP:resolve_bound_vars.rs-2093 */                                 // This will error in HIR lowering.
/* FP:resolve_bound_vars.rs-2094 */                                 self.tcx.dcx().span_delayed_bug(
/* FP:resolve_bound_vars.rs-2095 */                                     path.span,
/* FP:resolve_bound_vars.rs-2096 */                                     "ambiguous resolution for RTN path",
/* FP:resolve_bound_vars.rs-2097 */                                 );
/* FP:resolve_bound_vars.rs-2098 */                                 return;
/* FP:resolve_bound_vars.rs-2099 */                             }
/* FP:resolve_bound_vars.rs-2100 */                         }
/* FP:resolve_bound_vars.rs-2101 */ 
/* FP:resolve_bound_vars.rs-2102 */                         (bound_vars, assoc_item.def_id, item_segment)
/* FP:resolve_bound_vars.rs-2103 */                     }
/* FP:resolve_bound_vars.rs-2104 */                     // If we have a self type alias (in an impl), try to resolve an
/* FP:resolve_bound_vars.rs-2105 */                     // associated item from one of the supertraits of the impl's trait.
/* FP:resolve_bound_vars.rs-2106 */                     Res::SelfTyAlias { alias_to: impl_def_id, is_trait_impl: true, .. } => {
/* FP:resolve_bound_vars.rs-2107 */                         let hir::ItemKind::Impl(hir::Impl { of_trait: Some(of_trait), .. }) = self
/* FP:resolve_bound_vars.rs-2108 */                             .tcx
/* FP:resolve_bound_vars.rs-2109 */                             .hir_node_by_def_id(impl_def_id.expect_local())
/* FP:resolve_bound_vars.rs-2110 */                             .expect_item()
/* FP:resolve_bound_vars.rs-2111 */                             .kind
/* FP:resolve_bound_vars.rs-2112 */                         else {
/* FP:resolve_bound_vars.rs-2113 */                             return;
/* FP:resolve_bound_vars.rs-2114 */                         };
/* FP:resolve_bound_vars.rs-2115 */                         let Some(trait_def_id) = of_trait.trait_ref.trait_def_id() else {
/* FP:resolve_bound_vars.rs-2116 */                             return;
/* FP:resolve_bound_vars.rs-2117 */                         };
/* FP:resolve_bound_vars.rs-2118 */                         let Some((bound_vars, assoc_item)) = BoundVarContext::supertrait_hrtb_vars(
/* FP:resolve_bound_vars.rs-2119 */                             self.tcx,
/* FP:resolve_bound_vars.rs-2120 */                             trait_def_id,
/* FP:resolve_bound_vars.rs-2121 */                             item_segment.ident,
/* FP:resolve_bound_vars.rs-2122 */                             ty::AssocTag::Fn,
/* FP:resolve_bound_vars.rs-2123 */                         ) else {
/* FP:resolve_bound_vars.rs-2124 */                             return;
/* FP:resolve_bound_vars.rs-2125 */                         };
/* FP:resolve_bound_vars.rs-2126 */                         (bound_vars, assoc_item.def_id, item_segment)
/* FP:resolve_bound_vars.rs-2127 */                     }
/* FP:resolve_bound_vars.rs-2128 */                     _ => return,
/* FP:resolve_bound_vars.rs-2129 */                 }
/* FP:resolve_bound_vars.rs-2130 */             }
/* FP:resolve_bound_vars.rs-2131 */ 
/* FP:resolve_bound_vars.rs-2132 */             _ => return,
/* FP:resolve_bound_vars.rs-2133 */         };
/* FP:resolve_bound_vars.rs-2134 */ 
/* FP:resolve_bound_vars.rs-2135 */         // Append the early-bound vars on the function, and then the late-bound ones.
/* FP:resolve_bound_vars.rs-2136 */         // We actually turn type parameters into higher-ranked types here, but we
/* FP:resolve_bound_vars.rs-2137 */         // deny them later in HIR lowering.
/* FP:resolve_bound_vars.rs-2138 */         bound_vars.extend(
/* FP:resolve_bound_vars.rs-2139 */             self.tcx
/* FP:resolve_bound_vars.rs-2140 */                 .generics_of(item_def_id)
/* FP:resolve_bound_vars.rs-2141 */                 .own_params
/* FP:resolve_bound_vars.rs-2142 */                 .iter()
/* FP:resolve_bound_vars.rs-2143 */                 .map(|param| generic_param_def_as_bound_arg(param)),
/* FP:resolve_bound_vars.rs-2144 */         );
/* FP:resolve_bound_vars.rs-2145 */         bound_vars.extend(self.tcx.fn_sig(item_def_id).instantiate_identity().bound_vars());
/* FP:resolve_bound_vars.rs-2146 */ 
/* FP:resolve_bound_vars.rs-2147 */         // SUBTLE: Stash the old bound vars onto the *item segment* before appending
/* FP:resolve_bound_vars.rs-2148 */         // the new bound vars. We do this because we need to know how many bound vars
/* FP:resolve_bound_vars.rs-2149 */         // are present on the binder explicitly (i.e. not return-type-notation vars)
/* FP:resolve_bound_vars.rs-2150 */         // to do bound var shifting correctly in HIR lowering.
/* FP:resolve_bound_vars.rs-2151 */         //
/* FP:resolve_bound_vars.rs-2152 */         // For example, in `where for<'a> <T as Trait<'a>>::method(..): Other`,
/* FP:resolve_bound_vars.rs-2153 */         // the `late_bound_vars` of the where clause predicate (i.e. this HIR ty's
/* FP:resolve_bound_vars.rs-2154 */         // parent) will include `'a` AND all the early- and late-bound vars of the
/* FP:resolve_bound_vars.rs-2155 */         // method. But when lowering the RTN type, we just want the list of vars
/* FP:resolve_bound_vars.rs-2156 */         // we used to resolve the trait ref. We explicitly stored those back onto
/* FP:resolve_bound_vars.rs-2157 */         // the item segment, since there's no other good place to put them.
/* FP:resolve_bound_vars.rs-2158 */         //
/* FP:resolve_bound_vars.rs-2159 */         // See where these vars are used in `HirTyLowerer::lower_ty_maybe_return_type_notation`.
/* FP:resolve_bound_vars.rs-2160 */         // And this is exercised in:
/* FP:resolve_bound_vars.rs-2161 */         // `tests/ui/associated-type-bounds/return-type-notation/higher-ranked-bound-works.rs`.
/* FP:resolve_bound_vars.rs-2162 */         let existing_bound_vars = self.rbv.late_bound_vars.get_mut(&hir_id.local_id).unwrap();
/* FP:resolve_bound_vars.rs-2163 */         let existing_bound_vars_saved = existing_bound_vars.clone();
/* FP:resolve_bound_vars.rs-2164 */         existing_bound_vars.extend(bound_vars);
/* FP:resolve_bound_vars.rs-2165 */         self.record_late_bound_vars(item_segment.hir_id, existing_bound_vars_saved);
/* FP:resolve_bound_vars.rs-2166 */     }
/* FP:resolve_bound_vars.rs-2167 */ 
/* FP:resolve_bound_vars.rs-2168 */     /// Walk the generics of the item for a trait bound whose self type
/* FP:resolve_bound_vars.rs-2169 */     /// corresponds to the expected res, and return the trait def id.
/* FP:resolve_bound_vars.rs-2170 */     fn for_each_trait_bound_on_res(&self, expected_res: Res) -> impl Iterator<Item = DefId> {
/* FP:resolve_bound_vars.rs-2171 */         gen move {
/* FP:resolve_bound_vars.rs-2172 */             let mut scope = self.scope;
/* FP:resolve_bound_vars.rs-2173 */             loop {
/* FP:resolve_bound_vars.rs-2174 */                 let hir_id = match *scope {
/* FP:resolve_bound_vars.rs-2175 */                     Scope::Binder { hir_id, .. } => Some(hir_id),
/* FP:resolve_bound_vars.rs-2176 */                     Scope::Root { opt_parent_item: Some(parent_def_id) } => {
/* FP:resolve_bound_vars.rs-2177 */                         Some(self.tcx.local_def_id_to_hir_id(parent_def_id))
/* FP:resolve_bound_vars.rs-2178 */                     }
/* FP:resolve_bound_vars.rs-2179 */                     Scope::Body { .. }
/* FP:resolve_bound_vars.rs-2180 */                     | Scope::ObjectLifetimeDefault { .. }
/* FP:resolve_bound_vars.rs-2181 */                     | Scope::Supertrait { .. }
/* FP:resolve_bound_vars.rs-2182 */                     | Scope::TraitRefBoundary { .. }
/* FP:resolve_bound_vars.rs-2183 */                     | Scope::LateBoundary { .. }
/* FP:resolve_bound_vars.rs-2184 */                     | Scope::Opaque { .. }
/* FP:resolve_bound_vars.rs-2185 */                     | Scope::Root { opt_parent_item: None } => None,
/* FP:resolve_bound_vars.rs-2186 */                 };
/* FP:resolve_bound_vars.rs-2187 */ 
/* FP:resolve_bound_vars.rs-2188 */                 if let Some(hir_id) = hir_id {
/* FP:resolve_bound_vars.rs-2189 */                     let node = self.tcx.hir_node(hir_id);
/* FP:resolve_bound_vars.rs-2190 */                     // If this is a `Self` bound in a trait, yield the trait itself.
/* FP:resolve_bound_vars.rs-2191 */                     // Specifically, we don't need to look at any supertraits since
/* FP:resolve_bound_vars.rs-2192 */                     // we already do that in `BoundVarContext::supertrait_hrtb_vars`.
/* FP:resolve_bound_vars.rs-2193 */                     if let Res::SelfTyParam { trait_: _ } = expected_res
/* FP:resolve_bound_vars.rs-2194 */                         && let hir::Node::Item(item) = node
/* FP:resolve_bound_vars.rs-2195 */                         && let hir::ItemKind::Trait(..) = item.kind
/* FP:resolve_bound_vars.rs-2196 */                     {
/* FP:resolve_bound_vars.rs-2197 */                         // Yield the trait's def id. Supertraits will be
/* FP:resolve_bound_vars.rs-2198 */                         // elaborated from that.
/* FP:resolve_bound_vars.rs-2199 */                         yield item.owner_id.def_id.to_def_id();
/* FP:resolve_bound_vars.rs-2200 */                     } else if let Some(generics) = node.generics() {
/* FP:resolve_bound_vars.rs-2201 */                         for pred in generics.predicates {
/* FP:resolve_bound_vars.rs-2202 */                             let hir::WherePredicateKind::BoundPredicate(pred) = pred.kind else {
/* FP:resolve_bound_vars.rs-2203 */                                 continue;
/* FP:resolve_bound_vars.rs-2204 */                             };
/* FP:resolve_bound_vars.rs-2205 */                             let hir::TyKind::Path(hir::QPath::Resolved(None, bounded_path)) =
/* FP:resolve_bound_vars.rs-2206 */                                 pred.bounded_ty.kind
/* FP:resolve_bound_vars.rs-2207 */                             else {
/* FP:resolve_bound_vars.rs-2208 */                                 continue;
/* FP:resolve_bound_vars.rs-2209 */                             };
/* FP:resolve_bound_vars.rs-2210 */                             // Match the expected res.
/* FP:resolve_bound_vars.rs-2211 */                             if bounded_path.res != expected_res {
/* FP:resolve_bound_vars.rs-2212 */                                 continue;
/* FP:resolve_bound_vars.rs-2213 */                             }
/* FP:resolve_bound_vars.rs-2214 */                             for pred in pred.bounds {
/* FP:resolve_bound_vars.rs-2215 */                                 match pred {
/* FP:resolve_bound_vars.rs-2216 */                                     hir::GenericBound::Trait(poly_trait_ref) => {
/* FP:resolve_bound_vars.rs-2217 */                                         if let Some(def_id) =
/* FP:resolve_bound_vars.rs-2218 */                                             poly_trait_ref.trait_ref.trait_def_id()
/* FP:resolve_bound_vars.rs-2219 */                                         {
/* FP:resolve_bound_vars.rs-2220 */                                             yield def_id;
/* FP:resolve_bound_vars.rs-2221 */                                         }
/* FP:resolve_bound_vars.rs-2222 */                                     }
/* FP:resolve_bound_vars.rs-2223 */                                     hir::GenericBound::Outlives(_)
/* FP:resolve_bound_vars.rs-2224 */                                     | hir::GenericBound::Use(_, _) => {}
/* FP:resolve_bound_vars.rs-2225 */                                 }
/* FP:resolve_bound_vars.rs-2226 */                             }
/* FP:resolve_bound_vars.rs-2227 */                         }
/* FP:resolve_bound_vars.rs-2228 */                     }
/* FP:resolve_bound_vars.rs-2229 */                 }
/* FP:resolve_bound_vars.rs-2230 */ 
/* FP:resolve_bound_vars.rs-2231 */                 match *scope {
/* FP:resolve_bound_vars.rs-2232 */                     Scope::Binder { s, .. }
/* FP:resolve_bound_vars.rs-2233 */                     | Scope::Body { s, .. }
/* FP:resolve_bound_vars.rs-2234 */                     | Scope::ObjectLifetimeDefault { s, .. }
/* FP:resolve_bound_vars.rs-2235 */                     | Scope::Supertrait { s, .. }
/* FP:resolve_bound_vars.rs-2236 */                     | Scope::TraitRefBoundary { s }
/* FP:resolve_bound_vars.rs-2237 */                     | Scope::LateBoundary { s, .. }
/* FP:resolve_bound_vars.rs-2238 */                     | Scope::Opaque { s, .. } => {
/* FP:resolve_bound_vars.rs-2239 */                         scope = s;
/* FP:resolve_bound_vars.rs-2240 */                     }
/* FP:resolve_bound_vars.rs-2241 */                     Scope::Root { .. } => break,
/* FP:resolve_bound_vars.rs-2242 */                 }
/* FP:resolve_bound_vars.rs-2243 */             }
/* FP:resolve_bound_vars.rs-2244 */         }
/* FP:resolve_bound_vars.rs-2245 */     }
/* FP:resolve_bound_vars.rs-2246 */ }
/* FP:resolve_bound_vars.rs-2247 */ 
/* FP:resolve_bound_vars.rs-2248 */ /// Detects late-bound lifetimes and inserts them into
/* FP:resolve_bound_vars.rs-2249 */ /// `late_bound`.
/* FP:resolve_bound_vars.rs-2250 */ ///
/* FP:resolve_bound_vars.rs-2251 */ /// A region declared on a fn is **late-bound** if:
/* FP:resolve_bound_vars.rs-2252 */ /// - it is constrained by an argument type;
/* FP:resolve_bound_vars.rs-2253 */ /// - it does not appear in a where-clause.
/* FP:resolve_bound_vars.rs-2254 */ ///
/* FP:resolve_bound_vars.rs-2255 */ /// "Constrained" basically means that it appears in any type but
/* FP:resolve_bound_vars.rs-2256 */ /// not amongst the inputs to a projection. In other words, `<&'a
/* FP:resolve_bound_vars.rs-2257 */ /// T as Trait<''b>>::Foo` does not constrain `'a` or `'b`.
/* FP:resolve_bound_vars.rs-2258 */ fn is_late_bound_map(
/* FP:resolve_bound_vars.rs-2259 */     tcx: TyCtxt<'_>,
/* FP:resolve_bound_vars.rs-2260 */     owner_id: hir::OwnerId,
/* FP:resolve_bound_vars.rs-2261 */ ) -> Option<&FxIndexSet<hir::ItemLocalId>> {
/* FP:resolve_bound_vars.rs-2262 */     let sig = tcx.hir_fn_sig_by_hir_id(owner_id.into())?;
/* FP:resolve_bound_vars.rs-2263 */     let generics = tcx.hir_get_generics(owner_id.def_id)?;
/* FP:resolve_bound_vars.rs-2264 */ 
/* FP:resolve_bound_vars.rs-2265 */     let mut late_bound = FxIndexSet::default();
/* FP:resolve_bound_vars.rs-2266 */ 
/* FP:resolve_bound_vars.rs-2267 */     let mut constrained_by_input = ConstrainedCollector { regions: Default::default(), tcx };
/* FP:resolve_bound_vars.rs-2268 */     for arg_ty in sig.decl.inputs {
/* FP:resolve_bound_vars.rs-2269 */         constrained_by_input.visit_ty_unambig(arg_ty);
/* FP:resolve_bound_vars.rs-2270 */     }
/* FP:resolve_bound_vars.rs-2271 */ 
/* FP:resolve_bound_vars.rs-2272 */     let mut appears_in_output =
/* FP:resolve_bound_vars.rs-2273 */         AllCollector { has_fully_capturing_opaque: false, regions: Default::default() };
/* FP:resolve_bound_vars.rs-2274 */     intravisit::walk_fn_ret_ty(&mut appears_in_output, &sig.decl.output);
/* FP:resolve_bound_vars.rs-2275 */     if appears_in_output.has_fully_capturing_opaque {
/* FP:resolve_bound_vars.rs-2276 */         appears_in_output.regions.extend(generics.params.iter().map(|param| param.def_id));
/* FP:resolve_bound_vars.rs-2277 */     }
/* FP:resolve_bound_vars.rs-2278 */ 
/* FP:resolve_bound_vars.rs-2279 */     debug!(?constrained_by_input.regions);
/* FP:resolve_bound_vars.rs-2280 */ 
/* FP:resolve_bound_vars.rs-2281 */     // Walk the lifetimes that appear in where clauses.
/* FP:resolve_bound_vars.rs-2282 */     //
/* FP:resolve_bound_vars.rs-2283 */     // Subtle point: because we disallow nested bindings, we can just
/* FP:resolve_bound_vars.rs-2284 */     // ignore binders here and scrape up all names we see.
/* FP:resolve_bound_vars.rs-2285 */     let mut appears_in_where_clause =
/* FP:resolve_bound_vars.rs-2286 */         AllCollector { has_fully_capturing_opaque: true, regions: Default::default() };
/* FP:resolve_bound_vars.rs-2287 */     appears_in_where_clause.visit_generics(generics);
/* FP:resolve_bound_vars.rs-2288 */     debug!(?appears_in_where_clause.regions);
/* FP:resolve_bound_vars.rs-2289 */ 
/* FP:resolve_bound_vars.rs-2290 */     // Late bound regions are those that:
/* FP:resolve_bound_vars.rs-2291 */     // - appear in the inputs
/* FP:resolve_bound_vars.rs-2292 */     // - do not appear in the where-clauses
/* FP:resolve_bound_vars.rs-2293 */     // - are not implicitly captured by `impl Trait`
/* FP:resolve_bound_vars.rs-2294 */     for param in generics.params {
/* FP:resolve_bound_vars.rs-2295 */         match param.kind {
/* FP:resolve_bound_vars.rs-2296 */             hir::GenericParamKind::Lifetime { .. } => { /* fall through */ }
/* FP:resolve_bound_vars.rs-2297 */ 
/* FP:resolve_bound_vars.rs-2298 */             // Neither types nor consts are late-bound.
/* FP:resolve_bound_vars.rs-2299 */             hir::GenericParamKind::Type { .. } | hir::GenericParamKind::Const { .. } => continue,
/* FP:resolve_bound_vars.rs-2300 */         }
/* FP:resolve_bound_vars.rs-2301 */ 
/* FP:resolve_bound_vars.rs-2302 */         // appears in the where clauses? early-bound.
/* FP:resolve_bound_vars.rs-2303 */         if appears_in_where_clause.regions.contains(&param.def_id) {
/* FP:resolve_bound_vars.rs-2304 */             continue;
/* FP:resolve_bound_vars.rs-2305 */         }
/* FP:resolve_bound_vars.rs-2306 */ 
/* FP:resolve_bound_vars.rs-2307 */         // does not appear in the inputs, but appears in the return type? early-bound.
/* FP:resolve_bound_vars.rs-2308 */         if !constrained_by_input.regions.contains(&param.def_id)
/* FP:resolve_bound_vars.rs-2309 */             && appears_in_output.regions.contains(&param.def_id)
/* FP:resolve_bound_vars.rs-2310 */         {
/* FP:resolve_bound_vars.rs-2311 */             continue;
/* FP:resolve_bound_vars.rs-2312 */         }
/* FP:resolve_bound_vars.rs-2313 */ 
/* FP:resolve_bound_vars.rs-2314 */         debug!("lifetime {:?} with id {:?} is late-bound", param.name.ident(), param.def_id);
/* FP:resolve_bound_vars.rs-2315 */ 
/* FP:resolve_bound_vars.rs-2316 */         let inserted = late_bound.insert(param.hir_id.local_id);
/* FP:resolve_bound_vars.rs-2317 */         assert!(inserted, "visited lifetime {:?} twice", param.def_id);
/* FP:resolve_bound_vars.rs-2318 */     }
/* FP:resolve_bound_vars.rs-2319 */ 
/* FP:resolve_bound_vars.rs-2320 */     debug!(?late_bound);
/* FP:resolve_bound_vars.rs-2321 */     return Some(tcx.arena.alloc(late_bound));
/* FP:resolve_bound_vars.rs-2322 */ 
/* FP:resolve_bound_vars.rs-2323 */     /// Visits a `ty::Ty` collecting information about what generic parameters are constrained.
/* FP:resolve_bound_vars.rs-2324 */     ///
/* FP:resolve_bound_vars.rs-2325 */     /// The visitor does not operate on `hir::Ty` so that it can be called on the rhs of a `type Alias<...> = ...;`
/* FP:resolve_bound_vars.rs-2326 */     /// which may live in a separate crate so there would not be any hir available. Instead we use the `type_of`
/* FP:resolve_bound_vars.rs-2327 */     /// query to obtain a `ty::Ty` which will be present even in cross crate scenarios. It also naturally
/* FP:resolve_bound_vars.rs-2328 */     /// handles cycle detection as we go through the query system.
/* FP:resolve_bound_vars.rs-2329 */     ///
/* FP:resolve_bound_vars.rs-2330 */     /// This is necessary in the first place for the following case:
/* FP:resolve_bound_vars.rs-2331 */     /// ```rust,ignore (pseudo-Rust)
/* FP:resolve_bound_vars.rs-2332 */     /// type Alias<'a, T> = <T as Trait<'a>>::Assoc;
/* FP:resolve_bound_vars.rs-2333 */     /// fn foo<'a>(_: Alias<'a, ()>) -> Alias<'a, ()> { ... }
/* FP:resolve_bound_vars.rs-2334 */     /// ```
/* FP:resolve_bound_vars.rs-2335 */     ///
/* FP:resolve_bound_vars.rs-2336 */     /// If we conservatively considered `'a` unconstrained then we could break users who had written code before
/* FP:resolve_bound_vars.rs-2337 */     /// we started correctly handling aliases. If we considered `'a` constrained then it would become late bound
/* FP:resolve_bound_vars.rs-2338 */     /// causing an error during HIR ty lowering as the `'a` is not constrained by the input type `<() as Trait<'a>>::Assoc`
/* FP:resolve_bound_vars.rs-2339 */     /// but appears in the output type `<() as Trait<'a>>::Assoc`.
/* FP:resolve_bound_vars.rs-2340 */     ///
/* FP:resolve_bound_vars.rs-2341 */     /// We must therefore "look into" the `Alias` to see whether we should consider `'a` constrained or not.
/* FP:resolve_bound_vars.rs-2342 */     ///
/* FP:resolve_bound_vars.rs-2343 */     /// See #100508 #85533 #47511 for additional context
/* FP:resolve_bound_vars.rs-2344 */     struct ConstrainedCollectorPostHirTyLowering {
/* FP:resolve_bound_vars.rs-2345 */         arg_is_constrained: Box<[bool]>,
/* FP:resolve_bound_vars.rs-2346 */     }
/* FP:resolve_bound_vars.rs-2347 */ 
/* FP:resolve_bound_vars.rs-2348 */     use ty::Ty;
/* FP:resolve_bound_vars.rs-2349 */     impl<'tcx> TypeVisitor<TyCtxt<'tcx>> for ConstrainedCollectorPostHirTyLowering {
/* FP:resolve_bound_vars.rs-2350 */         fn visit_ty(&mut self, t: Ty<'tcx>) {
/* FP:resolve_bound_vars.rs-2351 */             match t.kind() {
/* FP:resolve_bound_vars.rs-2352 */                 ty::Param(param_ty) => {
/* FP:resolve_bound_vars.rs-2353 */                     self.arg_is_constrained[param_ty.index as usize] = true;
/* FP:resolve_bound_vars.rs-2354 */                 }
/* FP:resolve_bound_vars.rs-2355 */                 ty::Alias(ty::Projection | ty::Inherent, _) => return,
/* FP:resolve_bound_vars.rs-2356 */                 _ => (),
/* FP:resolve_bound_vars.rs-2357 */             }
/* FP:resolve_bound_vars.rs-2358 */             t.super_visit_with(self)
/* FP:resolve_bound_vars.rs-2359 */         }
/* FP:resolve_bound_vars.rs-2360 */ 
/* FP:resolve_bound_vars.rs-2361 */         fn visit_const(&mut self, _: ty::Const<'tcx>) {}
/* FP:resolve_bound_vars.rs-2362 */ 
/* FP:resolve_bound_vars.rs-2363 */         fn visit_region(&mut self, r: ty::Region<'tcx>) {
/* FP:resolve_bound_vars.rs-2364 */             debug!("r={:?}", r.kind());
/* FP:resolve_bound_vars.rs-2365 */             if let ty::RegionKind::ReEarlyParam(region) = r.kind() {
/* FP:resolve_bound_vars.rs-2366 */                 self.arg_is_constrained[region.index as usize] = true;
/* FP:resolve_bound_vars.rs-2367 */             }
/* FP:resolve_bound_vars.rs-2368 */         }
/* FP:resolve_bound_vars.rs-2369 */     }
/* FP:resolve_bound_vars.rs-2370 */ 
/* FP:resolve_bound_vars.rs-2371 */     struct ConstrainedCollector<'tcx> {
/* FP:resolve_bound_vars.rs-2372 */         tcx: TyCtxt<'tcx>,
/* FP:resolve_bound_vars.rs-2373 */         regions: FxHashSet<LocalDefId>,
/* FP:resolve_bound_vars.rs-2374 */     }
/* FP:resolve_bound_vars.rs-2375 */ 
/* FP:resolve_bound_vars.rs-2376 */     impl<'v> Visitor<'v> for ConstrainedCollector<'_> {
/* FP:resolve_bound_vars.rs-2377 */         fn visit_ty(&mut self, ty: &'v hir::Ty<'v, AmbigArg>) {
/* FP:resolve_bound_vars.rs-2378 */             match ty.kind {
/* FP:resolve_bound_vars.rs-2379 */                 hir::TyKind::Path(
/* FP:resolve_bound_vars.rs-2380 */                     hir::QPath::Resolved(Some(_), _) | hir::QPath::TypeRelative(..),
/* FP:resolve_bound_vars.rs-2381 */                 ) => {
/* FP:resolve_bound_vars.rs-2382 */                     // ignore lifetimes appearing in associated type
/* FP:resolve_bound_vars.rs-2383 */                     // projections, as they are not *constrained*
/* FP:resolve_bound_vars.rs-2384 */                     // (defined above)
/* FP:resolve_bound_vars.rs-2385 */                 }
/* FP:resolve_bound_vars.rs-2386 */ 
/* FP:resolve_bound_vars.rs-2387 */                 hir::TyKind::Path(hir::QPath::Resolved(
/* FP:resolve_bound_vars.rs-2388 */                     None,
/* FP:resolve_bound_vars.rs-2389 */                     hir::Path { res: Res::Def(DefKind::TyAlias, alias_def), segments, span },
/* FP:resolve_bound_vars.rs-2390 */                 )) => {
/* FP:resolve_bound_vars.rs-2391 */                     // See comments on `ConstrainedCollectorPostHirTyLowering` for why this arm does not
/* FP:resolve_bound_vars.rs-2392 */                     // just consider args to be unconstrained.
/* FP:resolve_bound_vars.rs-2393 */                     let generics = self.tcx.generics_of(alias_def);
/* FP:resolve_bound_vars.rs-2394 */                     let mut walker = ConstrainedCollectorPostHirTyLowering {
/* FP:resolve_bound_vars.rs-2395 */                         arg_is_constrained: vec![false; generics.own_params.len()]
/* FP:resolve_bound_vars.rs-2396 */                             .into_boxed_slice(),
/* FP:resolve_bound_vars.rs-2397 */                     };
/* FP:resolve_bound_vars.rs-2398 */                     walker.visit_ty(self.tcx.type_of(alias_def).instantiate_identity());
/* FP:resolve_bound_vars.rs-2399 */ 
/* FP:resolve_bound_vars.rs-2400 */                     match segments.last() {
/* FP:resolve_bound_vars.rs-2401 */                         Some(hir::PathSegment { args: Some(args), .. }) => {
/* FP:resolve_bound_vars.rs-2402 */                             let tcx = self.tcx;
/* FP:resolve_bound_vars.rs-2403 */                             for constrained_arg in
/* FP:resolve_bound_vars.rs-2404 */                                 args.args.iter().enumerate().flat_map(|(n, arg)| {
/* FP:resolve_bound_vars.rs-2405 */                                     match walker.arg_is_constrained.get(n) {
/* FP:resolve_bound_vars.rs-2406 */                                         Some(true) => Some(arg),
/* FP:resolve_bound_vars.rs-2407 */                                         Some(false) => None,
/* FP:resolve_bound_vars.rs-2408 */                                         None => {
/* FP:resolve_bound_vars.rs-2409 */                                             tcx.dcx().span_delayed_bug(
/* FP:resolve_bound_vars.rs-2410 */                                                 *span,
/* FP:resolve_bound_vars.rs-2411 */                                                 format!(
/* FP:resolve_bound_vars.rs-2412 */                                                     "Incorrect generic arg count for alias {alias_def:?}"
/* FP:resolve_bound_vars.rs-2413 */                                                 ),
/* FP:resolve_bound_vars.rs-2414 */                                             );
/* FP:resolve_bound_vars.rs-2415 */                                             None
/* FP:resolve_bound_vars.rs-2416 */                                         }
/* FP:resolve_bound_vars.rs-2417 */                                     }
/* FP:resolve_bound_vars.rs-2418 */                                 })
/* FP:resolve_bound_vars.rs-2419 */                             {
/* FP:resolve_bound_vars.rs-2420 */                                 self.visit_generic_arg(constrained_arg);
/* FP:resolve_bound_vars.rs-2421 */                             }
/* FP:resolve_bound_vars.rs-2422 */                         }
/* FP:resolve_bound_vars.rs-2423 */                         Some(_) => (),
/* FP:resolve_bound_vars.rs-2424 */                         None => bug!("Path with no segments or self type"),
/* FP:resolve_bound_vars.rs-2425 */                     }
/* FP:resolve_bound_vars.rs-2426 */                 }
/* FP:resolve_bound_vars.rs-2427 */ 
/* FP:resolve_bound_vars.rs-2428 */                 hir::TyKind::Path(hir::QPath::Resolved(None, path)) => {
/* FP:resolve_bound_vars.rs-2429 */                     // consider only the lifetimes on the final
/* FP:resolve_bound_vars.rs-2430 */                     // segment; I am not sure it's even currently
/* FP:resolve_bound_vars.rs-2431 */                     // valid to have them elsewhere, but even if it
/* FP:resolve_bound_vars.rs-2432 */                     // is, those would be potentially inputs to
/* FP:resolve_bound_vars.rs-2433 */                     // projections
/* FP:resolve_bound_vars.rs-2434 */                     if let Some(last_segment) = path.segments.last() {
/* FP:resolve_bound_vars.rs-2435 */                         self.visit_path_segment(last_segment);
/* FP:resolve_bound_vars.rs-2436 */                     }
/* FP:resolve_bound_vars.rs-2437 */                 }
/* FP:resolve_bound_vars.rs-2438 */ 
/* FP:resolve_bound_vars.rs-2439 */                 _ => {
/* FP:resolve_bound_vars.rs-2440 */                     intravisit::walk_ty(self, ty);
/* FP:resolve_bound_vars.rs-2441 */                 }
/* FP:resolve_bound_vars.rs-2442 */             }
/* FP:resolve_bound_vars.rs-2443 */         }
/* FP:resolve_bound_vars.rs-2444 */ 
/* FP:resolve_bound_vars.rs-2445 */         fn visit_lifetime(&mut self, lifetime_ref: &'v hir::Lifetime) {
/* FP:resolve_bound_vars.rs-2446 */             if let hir::LifetimeKind::Param(def_id) = lifetime_ref.kind {
/* FP:resolve_bound_vars.rs-2447 */                 self.regions.insert(def_id);
/* FP:resolve_bound_vars.rs-2448 */             }
/* FP:resolve_bound_vars.rs-2449 */         }
/* FP:resolve_bound_vars.rs-2450 */     }
/* FP:resolve_bound_vars.rs-2451 */ 
/* FP:resolve_bound_vars.rs-2452 */     struct AllCollector {
/* FP:resolve_bound_vars.rs-2453 */         has_fully_capturing_opaque: bool,
/* FP:resolve_bound_vars.rs-2454 */         regions: FxHashSet<LocalDefId>,
/* FP:resolve_bound_vars.rs-2455 */     }
/* FP:resolve_bound_vars.rs-2456 */ 
/* FP:resolve_bound_vars.rs-2457 */     impl<'tcx> Visitor<'tcx> for AllCollector {
/* FP:resolve_bound_vars.rs-2458 */         fn visit_lifetime(&mut self, lifetime_ref: &'tcx hir::Lifetime) {
/* FP:resolve_bound_vars.rs-2459 */             if let hir::LifetimeKind::Param(def_id) = lifetime_ref.kind {
/* FP:resolve_bound_vars.rs-2460 */                 self.regions.insert(def_id);
/* FP:resolve_bound_vars.rs-2461 */             }
/* FP:resolve_bound_vars.rs-2462 */         }
/* FP:resolve_bound_vars.rs-2463 */ 
/* FP:resolve_bound_vars.rs-2464 */         fn visit_opaque_ty(&mut self, opaque: &'tcx hir::OpaqueTy<'tcx>) {
/* FP:resolve_bound_vars.rs-2465 */             if !self.has_fully_capturing_opaque {
/* FP:resolve_bound_vars.rs-2466 */                 self.has_fully_capturing_opaque = opaque_captures_all_in_scope_lifetimes(opaque);
/* FP:resolve_bound_vars.rs-2467 */             }
/* FP:resolve_bound_vars.rs-2468 */             intravisit::walk_opaque_ty(self, opaque);
/* FP:resolve_bound_vars.rs-2469 */         }
/* FP:resolve_bound_vars.rs-2470 */     }
/* FP:resolve_bound_vars.rs-2471 */ }
/* FP:resolve_bound_vars.rs-2472 */ 
/* FP:resolve_bound_vars.rs-2473 */ fn deny_non_region_late_bound(
/* FP:resolve_bound_vars.rs-2474 */     tcx: TyCtxt<'_>,
/* FP:resolve_bound_vars.rs-2475 */     bound_vars: &mut FxIndexMap<LocalDefId, ResolvedArg>,
/* FP:resolve_bound_vars.rs-2476 */     where_: &str,
/* FP:resolve_bound_vars.rs-2477 */ ) {
/* FP:resolve_bound_vars.rs-2478 */     let mut first = true;
/* FP:resolve_bound_vars.rs-2479 */ 
/* FP:resolve_bound_vars.rs-2480 */     for (var, arg) in bound_vars {
/* FP:resolve_bound_vars.rs-2481 */         let Node::GenericParam(param) = tcx.hir_node_by_def_id(*var) else {
/* FP:resolve_bound_vars.rs-2482 */             span_bug!(tcx.def_span(*var), "expected bound-var def-id to resolve to param");
/* FP:resolve_bound_vars.rs-2483 */         };
/* FP:resolve_bound_vars.rs-2484 */ 
/* FP:resolve_bound_vars.rs-2485 */         let what = match param.kind {
/* FP:resolve_bound_vars.rs-2486 */             hir::GenericParamKind::Type { .. } => "type",
/* FP:resolve_bound_vars.rs-2487 */             hir::GenericParamKind::Const { .. } => "const",
/* FP:resolve_bound_vars.rs-2488 */             hir::GenericParamKind::Lifetime { .. } => continue,
/* FP:resolve_bound_vars.rs-2489 */         };
/* FP:resolve_bound_vars.rs-2490 */ 
/* FP:resolve_bound_vars.rs-2491 */         let diag = tcx.dcx().struct_span_err(
/* FP:resolve_bound_vars.rs-2492 */             param.span,
/* FP:resolve_bound_vars.rs-2493 */             format!("late-bound {what} parameter not allowed on {where_}"),
/* FP:resolve_bound_vars.rs-2494 */         );
/* FP:resolve_bound_vars.rs-2495 */ 
/* FP:resolve_bound_vars.rs-2496 */         let guar = diag.emit_unless_delay(!tcx.features().non_lifetime_binders() || !first);
/* FP:resolve_bound_vars.rs-2497 */ 
/* FP:resolve_bound_vars.rs-2498 */         first = false;
/* FP:resolve_bound_vars.rs-2499 */         *arg = ResolvedArg::Error(guar);
/* FP:resolve_bound_vars.rs-2500 */     }
/* FP:resolve_bound_vars.rs-2501 */ }