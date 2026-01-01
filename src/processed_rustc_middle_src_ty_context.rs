/* FP:context.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_context_UNPARSEABLE_0001
/* FP:context.rs-0002 */ // Type context book-keeping.
/* FP:context.rs-0003 */ 
/* FP:context.rs-0004 */ #[allow(rustc::usage_of_ty_tykind)]
/* FP:context.rs-0005 */ 
/* FP:context.rs-0007 */ 
/* FP:context.rs-0008 */ use std::assert_matches::debug_assert_matches;
/* FP:context.rs-0009 */ use std::borrow::{Borrow, Cow};
/* FP:context.rs-0010 */ use std::cmp::Ordering;
/* FP:context.rs-0011 */ use std::env::VarError;
/* FP:context.rs-0012 */ use std::ffi::OsStr;
/* FP:context.rs-0013 */ use std::hash::{Hash, Hasher};
/* FP:context.rs-0014 */ use std::marker::{PhantomData, PointeeSized};
/* FP:context.rs-0015 */ use std::ops::{Bound, Deref};
/* FP:context.rs-0016 */ use std::sync::{Arc, OnceLock};
/* FP:context.rs-0017 */ use std::{fmt, iter, mem};
/* FP:context.rs-0018 */ 
/* FP:context.rs-0019 */ use crate::rustc_abi::{ExternAbi, FieldIdx, Layout, LayoutData, TargetDataLayout, VariantIdx};
/* FP:context.rs-0020 */ use rustc_ast as ast;
/* FP:context.rs-0021 */ use crate::rustc_data_structures::defer;
/* FP:context.rs-0022 */ use crate::rustc_data_structures::fingerprint::Fingerprint;
/* FP:context.rs-0023 */ use crate::rustc_data_structures::fx::FxHashMap;
/* FP:context.rs-0024 */ use crate::rustc_data_structures::intern::Interned;
/* FP:context.rs-0025 */ use crate::rustc_data_structures::jobserver::Proxy;
/* FP:context.rs-0026 */ use crate::rustc_data_structures::profiling::SelfProfilerRef;
/* FP:context.rs-0027 */ use crate::rustc_data_structures::sharded::{IntoPointer, ShardedHashMap};
/* FP:context.rs-0028 */ use crate::rustc_data_structures::stable_hasher::{HashStable, StableHasher};
/* FP:context.rs-0029 */ use crate::rustc_data_structures::steal::Steal;
/* FP:context.rs-0030 */ use crate::rustc_data_structures::sync::{
/* FP:context.rs-0031 */     self, DynSend, DynSync, FreezeReadGuard, Lock, RwLock, WorkerLocal,
/* FP:context.rs-0032 */ };
/* FP:context.rs-0033 */ use crate::rustc_complete::{
/* FP:context.rs-0034 */     Applicability, Diag, DiagCtxtHandle, ErrorGuaranteed, LintDiagnostic, LintEmitter, MultiSpan,
/* FP:context.rs-0035 */ };
/* FP:context.rs-0036 */ use crate::rustc_complete::attrs::AttributeKind;
/* FP:context.rs-0037 */ use crate::rustc_complete::def::{CtorKind, CtorOf, DefKind};
/* FP:context.rs-0038 */ use crate::rustc_complete::def_id::{CrateNum, DefId, LOCAL_CRATE, LocalDefId};
/* FP:context.rs-0039 */ use crate::rustc_complete::definitions::{DefPathData, Definitions, DisambiguatorState};
/* FP:context.rs-0040 */ use crate::rustc_complete::intravisit::VisitorExt;
/* FP:context.rs-0041 */ use crate::rustc_complete::lang_items::LangItem;
/* FP:context.rs-0042 */ use crate::rustc_complete::limit::Limit;
/* FP:context.rs-0043 */ use crate::rustc_complete::{self as hir, Attribute, HirId, Node, TraitCandidate, find_attr};
/* FP:context.rs-0044 */ use crate::rustc_index::IndexVec;
/* FP:context.rs-0045 */ use rustc_macros::{HashStable, TyDecodable, TyEncodable};
/* FP:context.rs-0046 */ use rustc_query_system::cache::WithDepNode;
/* FP:context.rs-0047 */ use rustc_query_system::dep_graph::DepNodeIndex;
/* FP:context.rs-0048 */ use rustc_query_system::ich::StableHashingContext;
/* FP:context.rs-0049 */ use crate::rustc_serialize::opaque::{FileEncodeResult, FileEncoder};
/* FP:context.rs-0050 */ use crate::rustc_complete::Session;
/* FP:context.rs-0051 */ use crate::rustc_complete::config::CrateType;
/* FP:context.rs-0052 */ use crate::rustc_complete::cstore::{CrateStoreDyn, Untracked};
/* FP:context.rs-0053 */ use crate::rustc_complete::lint::Lint;
/* FP:context.rs-0054 */ use crate::rustc_complete::def_id::{CRATE_DEF_ID, DefPathHash, StableCrateId};
/* FP:context.rs-0055 */ use crate::rustc_complete::{DUMMY_SP, Ident, Span, Symbol, kw, sym};
/* FP:context.rs-0056 */ use rustc_type_ir::TyKind::*;
/* FP:context.rs-0057 */ use rustc_type_ir::lang_items::{SolverAdtLangItem, SolverLangItem, SolverTraitLangItem};
/* FP:context.rs-0058 */ pub use rustc_type_ir::lift::Lift;
/* FP:context.rs-0059 */ use rustc_type_ir::{
/* FP:context.rs-0060 */     CollectAndApply, Interner, TypeFlags, TypeFoldable, WithCachedTypeInfo, elaborate, search_graph,
/* FP:context.rs-0061 */ };
/* FP:context.rs-0062 */ use tracing::{debug, instrument};
/* FP:context.rs-0063 */ 
/* FP:context.rs-0064 */ use crate::arena::Arena;
/* FP:context.rs-0065 */ use crate::dep_graph::{DepGraph, DepKindStruct};
/* FP:context.rs-0066 */ use crate::infer::canonical::{CanonicalParamEnvCache, CanonicalVarKind, CanonicalVarKinds};
/* FP:context.rs-0067 */ use crate::lint::lint_level;
/* FP:context.rs-0068 */ use crate::metadata::ModChild;
/* FP:context.rs-0069 */ use crate::middle::codegen_fn_attrs::{CodegenFnAttrs, TargetFeature};
/* FP:context.rs-0070 */ use crate::middle::resolve_bound_vars;
/* FP:context.rs-0071 */ use crate::mir::interpret::{self, Allocation, ConstAllocation};
/* FP:context.rs-0072 */ use crate::mir::{Body, Local, Place, PlaceElem, ProjectionKind, Promoted};
/* FP:context.rs-0073 */ use crate::query::plumbing::QuerySystem;
/* FP:context.rs-0074 */ use crate::query::{IntoQueryParam, LocalCrate, Providers, TyCtxtAt};
/* FP:context.rs-0075 */ use crate::thir::Thir;
/* FP:context.rs-0076 */ use crate::traits;
/* FP:context.rs-0077 */ use crate::traits::solve::{
/* FP:context.rs-0078 */     self, CanonicalInput, ExternalConstraints, ExternalConstraintsData, PredefinedOpaques,
/* FP:context.rs-0079 */     PredefinedOpaquesData, QueryResult, inspect,
/* FP:context.rs-0080 */ };
/* FP:context.rs-0081 */ use crate::ty::predicate::ExistentialPredicateStableCmpExt as _;
/* FP:context.rs-0082 */ use crate::ty::{
/* FP:context.rs-0083 */     self, AdtDef, AdtDefData, AdtKind, Binder, Clause, Clauses, Const, GenericArg, GenericArgs,
/* FP:context.rs-0084 */     GenericArgsRef, GenericParamDefKind, List, ListWithCachedTypeInfo, ParamConst, ParamTy,
/* FP:context.rs-0085 */     Pattern, PatternKind, PolyExistentialPredicate, PolyFnSig, Predicate, PredicateKind,
/* FP:context.rs-0086 */     PredicatePolarity, Region, RegionKind, ReprOptions, TraitObjectVisitor, Ty, TyKind, TyVid,
/* FP:context.rs-0087 */     ValTree, ValTreeKind, Visibility,
/* FP:context.rs-0088 */ };
/* FP:context.rs-0089 */ 
/* FP:context.rs-0090 */ #[allow(rustc::usage_of_ty_tykind)]
/* FP:context.rs-0091 */ impl<'tcx> Interner for TyCtxt<'tcx> {
/* FP:context.rs-0092 */     fn next_trait_solver_globally(self) -> bool {
/* FP:context.rs-0093 */         self.next_trait_solver_globally()
/* FP:context.rs-0094 */     }
/* FP:context.rs-0095 */ 
/* FP:context.rs-0096 */     type DefId = DefId;
/* FP:context.rs-0097 */     type LocalDefId = LocalDefId;
/* FP:context.rs-0098 */     type TraitId = DefId;
/* FP:context.rs-0099 */     type ForeignId = DefId;
/* FP:context.rs-0100 */     type FunctionId = DefId;
/* FP:context.rs-0101 */     type ClosureId = DefId;
/* FP:context.rs-0102 */     type CoroutineClosureId = DefId;
/* FP:context.rs-0103 */     type CoroutineId = DefId;
/* FP:context.rs-0104 */     type AdtId = DefId;
/* FP:context.rs-0105 */     type ImplId = DefId;
/* FP:context.rs-0106 */     type Span = Span;
/* FP:context.rs-0107 */ 
/* FP:context.rs-0108 */     type GenericArgs = ty::GenericArgsRef<'tcx>;
/* FP:context.rs-0109 */ 
/* FP:context.rs-0110 */     type GenericArgsSlice = &'tcx [ty::GenericArg<'tcx>];
/* FP:context.rs-0111 */     type GenericArg = ty::GenericArg<'tcx>;
/* FP:context.rs-0112 */     type Term = ty::Term<'tcx>;
/* FP:context.rs-0113 */     type BoundVarKinds = &'tcx List<ty::BoundVariableKind>;
/* FP:context.rs-0114 */ 
/* FP:context.rs-0115 */     type BoundVarKind = ty::BoundVariableKind;
/* FP:context.rs-0116 */     type PredefinedOpaques = solve::PredefinedOpaques<'tcx>;
/* FP:context.rs-0117 */ 
/* FP:context.rs-0118 */     fn mk_predefined_opaques_in_body(
/* FP:context.rs-0119 */         self,
/* FP:context.rs-0120 */         data: PredefinedOpaquesData<Self>,
/* FP:context.rs-0121 */     ) -> Self::PredefinedOpaques {
/* FP:context.rs-0122 */         self.mk_predefined_opaques_in_body(data)
/* FP:context.rs-0123 */     }
/* FP:context.rs-0124 */     type LocalDefIds = &'tcx ty::List<LocalDefId>;
/* FP:context.rs-0125 */     type CanonicalVarKinds = CanonicalVarKinds<'tcx>;
/* FP:context.rs-0126 */     fn mk_canonical_var_kinds(
/* FP:context.rs-0127 */         self,
/* FP:context.rs-0128 */         kinds: &[ty::CanonicalVarKind<Self>],
/* FP:context.rs-0129 */     ) -> Self::CanonicalVarKinds {
/* FP:context.rs-0130 */         self.mk_canonical_var_kinds(kinds)
/* FP:context.rs-0131 */     }
/* FP:context.rs-0132 */ 
/* FP:context.rs-0133 */     type ExternalConstraints = ExternalConstraints<'tcx>;
/* FP:context.rs-0134 */     fn mk_external_constraints(
/* FP:context.rs-0135 */         self,
/* FP:context.rs-0136 */         data: ExternalConstraintsData<Self>,
/* FP:context.rs-0137 */     ) -> ExternalConstraints<'tcx> {
/* FP:context.rs-0138 */         self.mk_external_constraints(data)
/* FP:context.rs-0139 */     }
/* FP:context.rs-0140 */     type DepNodeIndex = DepNodeIndex;
/* FP:context.rs-0141 */     fn with_cached_task<T>(self, task: impl FnOnce() -> T) -> (T, DepNodeIndex) {
/* FP:context.rs-0142 */         self.dep_graph.with_anon_task(self, crate::dep_graph::dep_kinds::TraitSelect, task)
/* FP:context.rs-0143 */     }
/* FP:context.rs-0144 */     type Ty = Ty<'tcx>;
/* FP:context.rs-0145 */     type Tys = &'tcx List<Ty<'tcx>>;
/* FP:context.rs-0146 */ 
/* FP:context.rs-0147 */     type FnInputTys = &'tcx [Ty<'tcx>];
/* FP:context.rs-0148 */     type ParamTy = ParamTy;
/* FP:context.rs-0149 */     type BoundTy = ty::BoundTy;
/* FP:context.rs-0150 */     type Symbol = Symbol;
/* FP:context.rs-0151 */ 
/* FP:context.rs-0152 */     type PlaceholderTy = ty::PlaceholderType;
/* FP:context.rs-0153 */     type ErrorGuaranteed = ErrorGuaranteed;
/* FP:context.rs-0154 */     type BoundExistentialPredicates = &'tcx List<PolyExistentialPredicate<'tcx>>;
/* FP:context.rs-0155 */ 
/* FP:context.rs-0156 */     type AllocId = crate::mir::interpret::AllocId;
/* FP:context.rs-0157 */     type Pat = Pattern<'tcx>;
/* FP:context.rs-0158 */     type PatList = &'tcx List<Pattern<'tcx>>;
/* FP:context.rs-0159 */     type Safety = hir::Safety;
/* FP:context.rs-0160 */     type Abi = ExternAbi;
/* FP:context.rs-0161 */     type Const = ty::Const<'tcx>;
/* FP:context.rs-0162 */     type PlaceholderConst = ty::PlaceholderConst;
/* FP:context.rs-0163 */ 
/* FP:context.rs-0164 */     type ParamConst = ty::ParamConst;
/* FP:context.rs-0165 */     type BoundConst = ty::BoundConst;
/* FP:context.rs-0166 */     type ValueConst = ty::Value<'tcx>;
/* FP:context.rs-0167 */     type ExprConst = ty::Expr<'tcx>;
/* FP:context.rs-0168 */     type ValTree = ty::ValTree<'tcx>;
/* FP:context.rs-0169 */ 
/* FP:context.rs-0170 */     type Region = Region<'tcx>;
/* FP:context.rs-0171 */     type EarlyParamRegion = ty::EarlyParamRegion;
/* FP:context.rs-0172 */     type LateParamRegion = ty::LateParamRegion;
/* FP:context.rs-0173 */     type BoundRegion = ty::BoundRegion;
/* FP:context.rs-0174 */     type PlaceholderRegion = ty::PlaceholderRegion;
/* FP:context.rs-0175 */ 
/* FP:context.rs-0176 */     type RegionAssumptions = &'tcx ty::List<ty::ArgOutlivesPredicate<'tcx>>;
/* FP:context.rs-0177 */ 
/* FP:context.rs-0178 */     type ParamEnv = ty::ParamEnv<'tcx>;
/* FP:context.rs-0179 */     type Predicate = Predicate<'tcx>;
/* FP:context.rs-0180 */ 
/* FP:context.rs-0181 */     type Clause = Clause<'tcx>;
/* FP:context.rs-0182 */     type Clauses = ty::Clauses<'tcx>;
/* FP:context.rs-0183 */ 
/* FP:context.rs-0184 */     type Tracked<T: fmt::Debug + Clone> = WithDepNode<T>;
/* FP:context.rs-0185 */     fn mk_tracked<T: fmt::Debug + Clone>(
/* FP:context.rs-0186 */         self,
/* FP:context.rs-0187 */         data: T,
/* FP:context.rs-0188 */         dep_node: DepNodeIndex,
/* FP:context.rs-0189 */     ) -> Self::Tracked<T> {
/* FP:context.rs-0190 */         WithDepNode::new(dep_node, data)
/* FP:context.rs-0191 */     }
/* FP:context.rs-0192 */     fn get_tracked<T: fmt::Debug + Clone>(self, tracked: &Self::Tracked<T>) -> T {
/* FP:context.rs-0193 */         tracked.get(self)
/* FP:context.rs-0194 */     }
/* FP:context.rs-0195 */ 
/* FP:context.rs-0196 */     fn with_global_cache<R>(self, f: impl FnOnce(&mut search_graph::GlobalCache<Self>) -> R) -> R {
/* FP:context.rs-0197 */         f(&mut *self.new_solver_evaluation_cache.lock())
/* FP:context.rs-0198 */     }
/* FP:context.rs-0199 */ 
/* FP:context.rs-0200 */     fn canonical_param_env_cache_get_or_insert<R>(
/* FP:context.rs-0201 */         self,
/* FP:context.rs-0202 */         param_env: ty::ParamEnv<'tcx>,
/* FP:context.rs-0203 */         f: impl FnOnce() -> ty::CanonicalParamEnvCacheEntry<Self>,
/* FP:context.rs-0204 */         from_entry: impl FnOnce(&ty::CanonicalParamEnvCacheEntry<Self>) -> R,
/* FP:context.rs-0205 */     ) -> R {
/* FP:context.rs-0206 */         let mut cache = self.new_solver_canonical_param_env_cache.lock();
/* FP:context.rs-0207 */         let entry = cache.entry(param_env).or_insert_with(f);
/* FP:context.rs-0208 */         from_entry(entry)
/* FP:context.rs-0209 */     }
/* FP:context.rs-0210 */ 
/* FP:context.rs-0211 */     fn evaluation_is_concurrent(&self) -> bool {
/* FP:context.rs-0212 */         self.sess.threads() > 1
/* FP:context.rs-0213 */     }
/* FP:context.rs-0214 */ 
/* FP:context.rs-0215 */     fn expand_abstract_consts<T: TypeFoldable<TyCtxt<'tcx>>>(self, t: T) -> T {
/* FP:context.rs-0216 */         self.expand_abstract_consts(t)
/* FP:context.rs-0217 */     }
/* FP:context.rs-0218 */ 
/* FP:context.rs-0219 */     type GenericsOf = &'tcx ty::Generics;
/* FP:context.rs-0220 */ 
/* FP:context.rs-0221 */     fn generics_of(self, def_id: DefId) -> &'tcx ty::Generics {
/* FP:context.rs-0222 */         self.generics_of(def_id)
/* FP:context.rs-0223 */     }
/* FP:context.rs-0224 */ 
/* FP:context.rs-0225 */     type VariancesOf = &'tcx [ty::Variance];
/* FP:context.rs-0226 */ 
/* FP:context.rs-0227 */     fn variances_of(self, def_id: DefId) -> Self::VariancesOf {
/* FP:context.rs-0228 */         self.variances_of(def_id)
/* FP:context.rs-0229 */     }
/* FP:context.rs-0230 */ 
/* FP:context.rs-0231 */     fn opt_alias_variances(
/* FP:context.rs-0232 */         self,
/* FP:context.rs-0233 */         kind: impl Into<ty::AliasTermKind>,
/* FP:context.rs-0234 */         def_id: DefId,
/* FP:context.rs-0235 */     ) -> Option<&'tcx [ty::Variance]> {
/* FP:context.rs-0236 */         self.opt_alias_variances(kind, def_id)
/* FP:context.rs-0237 */     }
/* FP:context.rs-0238 */ 
/* FP:context.rs-0239 */     fn type_of(self, def_id: DefId) -> ty::EarlyBinder<'tcx, Ty<'tcx>> {
/* FP:context.rs-0240 */         self.type_of(def_id)
/* FP:context.rs-0241 */     }
/* FP:context.rs-0242 */     fn type_of_opaque_hir_typeck(self, def_id: LocalDefId) -> ty::EarlyBinder<'tcx, Ty<'tcx>> {
/* FP:context.rs-0243 */         self.type_of_opaque_hir_typeck(def_id)
/* FP:context.rs-0244 */     }
/* FP:context.rs-0245 */ 
/* FP:context.rs-0246 */     type AdtDef = ty::AdtDef<'tcx>;
/* FP:context.rs-0247 */     fn adt_def(self, adt_def_id: DefId) -> Self::AdtDef {
/* FP:context.rs-0248 */         self.adt_def(adt_def_id)
/* FP:context.rs-0249 */     }
/* FP:context.rs-0250 */ 
/* FP:context.rs-0251 */     fn alias_ty_kind(self, alias: ty::AliasTy<'tcx>) -> ty::AliasTyKind {
/* FP:context.rs-0252 */         match self.def_kind(alias.def_id) {
/* FP:context.rs-0253 */             DefKind::AssocTy => {
/* FP:context.rs-0254 */                 if let DefKind::Impl { of_trait: false } = self.def_kind(self.parent(alias.def_id))
/* FP:context.rs-0255 */                 {
/* FP:context.rs-0256 */                     ty::Inherent
/* FP:context.rs-0257 */                 } else {
/* FP:context.rs-0258 */                     ty::Projection
/* FP:context.rs-0259 */                 }
/* FP:context.rs-0260 */             }
/* FP:context.rs-0261 */             DefKind::OpaqueTy => ty::Opaque,
/* FP:context.rs-0262 */             DefKind::TyAlias => ty::Free,
/* FP:context.rs-0263 */             kind => bug!("unexpected DefKind in AliasTy: {kind:?}"),
/* FP:context.rs-0264 */         }
/* FP:context.rs-0265 */     }
/* FP:context.rs-0266 */ 
/* FP:context.rs-0267 */     fn alias_term_kind(self, alias: ty::AliasTerm<'tcx>) -> ty::AliasTermKind {
/* FP:context.rs-0268 */         match self.def_kind(alias.def_id) {
/* FP:context.rs-0269 */             DefKind::AssocTy => {
/* FP:context.rs-0270 */                 if let DefKind::Impl { of_trait: false } = self.def_kind(self.parent(alias.def_id))
/* FP:context.rs-0271 */                 {
/* FP:context.rs-0272 */                     ty::AliasTermKind::InherentTy
/* FP:context.rs-0273 */                 } else {
/* FP:context.rs-0274 */                     ty::AliasTermKind::ProjectionTy
/* FP:context.rs-0275 */                 }
/* FP:context.rs-0276 */             }
/* FP:context.rs-0277 */             DefKind::AssocConst => {
/* FP:context.rs-0278 */                 if let DefKind::Impl { of_trait: false } = self.def_kind(self.parent(alias.def_id))
/* FP:context.rs-0279 */                 {
/* FP:context.rs-0280 */                     ty::AliasTermKind::InherentConst
/* FP:context.rs-0281 */                 } else {
/* FP:context.rs-0282 */                     ty::AliasTermKind::ProjectionConst
/* FP:context.rs-0283 */                 }
/* FP:context.rs-0284 */             }
/* FP:context.rs-0285 */             DefKind::OpaqueTy => ty::AliasTermKind::OpaqueTy,
/* FP:context.rs-0286 */             DefKind::TyAlias => ty::AliasTermKind::FreeTy,
/* FP:context.rs-0287 */             DefKind::Const => ty::AliasTermKind::FreeConst,
/* FP:context.rs-0288 */             DefKind::AnonConst | DefKind::Ctor(_, CtorKind::Const) => {
/* FP:context.rs-0289 */                 ty::AliasTermKind::UnevaluatedConst
/* FP:context.rs-0290 */             }
/* FP:context.rs-0291 */             kind => bug!("unexpected DefKind in AliasTy: {kind:?}"),
/* FP:context.rs-0292 */         }
/* FP:context.rs-0293 */     }
/* FP:context.rs-0294 */ 
/* FP:context.rs-0295 */     fn trait_ref_and_own_args_for_alias(
/* FP:context.rs-0296 */         self,
/* FP:context.rs-0297 */         def_id: DefId,
/* FP:context.rs-0298 */         args: ty::GenericArgsRef<'tcx>,
/* FP:context.rs-0299 */     ) -> (ty::TraitRef<'tcx>, &'tcx [ty::GenericArg<'tcx>]) {
/* FP:context.rs-0300 */         debug_assert_matches!(self.def_kind(def_id), DefKind::AssocTy | DefKind::AssocConst);
/* FP:context.rs-0301 */         let trait_def_id = self.parent(def_id);
/* FP:context.rs-0302 */         debug_assert_matches!(self.def_kind(trait_def_id), DefKind::Trait);
/* FP:context.rs-0303 */         let trait_ref = ty::TraitRef::from_assoc(self, trait_def_id, args);
/* FP:context.rs-0304 */         (trait_ref, &args[trait_ref.args.len()..])
/* FP:context.rs-0305 */     }
/* FP:context.rs-0306 */ 
/* FP:context.rs-0307 */     fn mk_args(self, args: &[Self::GenericArg]) -> ty::GenericArgsRef<'tcx> {
/* FP:context.rs-0308 */         self.mk_args(args)
/* FP:context.rs-0309 */     }
/* FP:context.rs-0310 */ 
/* FP:context.rs-0311 */     fn mk_args_from_iter<I, T>(self, args: I) -> T::Output
/* FP:context.rs-0312 */     where
/* FP:context.rs-0313 */         I: Iterator<Item = T>,
/* FP:context.rs-0314 */         T: CollectAndApply<Self::GenericArg, ty::GenericArgsRef<'tcx>>,
/* FP:context.rs-0315 */     {
/* FP:context.rs-0316 */         self.mk_args_from_iter(args)
/* FP:context.rs-0317 */     }
/* FP:context.rs-0318 */ 
/* FP:context.rs-0319 */     fn check_args_compatible(self, def_id: DefId, args: ty::GenericArgsRef<'tcx>) -> bool {
/* FP:context.rs-0320 */         self.check_args_compatible(def_id, args)
/* FP:context.rs-0321 */     }
/* FP:context.rs-0322 */ 
/* FP:context.rs-0323 */     fn debug_assert_args_compatible(self, def_id: DefId, args: ty::GenericArgsRef<'tcx>) {
/* FP:context.rs-0324 */         self.debug_assert_args_compatible(def_id, args);
/* FP:context.rs-0325 */     }
/* FP:context.rs-0326 */ 
/* FP:context.rs-0327 */     /// Assert that the args from an `ExistentialTraitRef` or `ExistentialProjection`
/* FP:context.rs-0328 */     /// are compatible with the `DefId`. Since we're missing a `Self` type, stick on
/* FP:context.rs-0329 */     /// a dummy self type and forward to `debug_assert_args_compatible`.
/* FP:context.rs-0330 */     fn debug_assert_existential_args_compatible(
/* FP:context.rs-0331 */         self,
/* FP:context.rs-0332 */         def_id: Self::DefId,
/* FP:context.rs-0333 */         args: Self::GenericArgs,
/* FP:context.rs-0334 */     ) {
/* FP:context.rs-0335 */         // FIXME: We could perhaps add a `skip: usize` to `debug_assert_args_compatible`
/* FP:context.rs-0336 */         // to avoid needing to reintern the set of args...
/* FP:context.rs-0337 */         if cfg!(debug_assertions) {
/* FP:context.rs-0338 */             self.debug_assert_args_compatible(
/* FP:context.rs-0339 */                 def_id,
/* FP:context.rs-0340 */                 self.mk_args_from_iter(
/* FP:context.rs-0341 */                     [self.types.trait_object_dummy_self.into()].into_iter().chain(args.iter()),
/* FP:context.rs-0342 */                 ),
/* FP:context.rs-0343 */             );
/* FP:context.rs-0344 */         }
/* FP:context.rs-0345 */     }
/* FP:context.rs-0346 */ 
/* FP:context.rs-0347 */     fn mk_type_list_from_iter<I, T>(self, args: I) -> T::Output
/* FP:context.rs-0348 */     where
/* FP:context.rs-0349 */         I: Iterator<Item = T>,
/* FP:context.rs-0350 */         T: CollectAndApply<Ty<'tcx>, &'tcx List<Ty<'tcx>>>,
/* FP:context.rs-0351 */     {
/* FP:context.rs-0352 */         self.mk_type_list_from_iter(args)
/* FP:context.rs-0353 */     }
/* FP:context.rs-0354 */ 
/* FP:context.rs-0355 */     fn parent(self, def_id: DefId) -> DefId {
/* FP:context.rs-0356 */         self.parent(def_id)
/* FP:context.rs-0357 */     }
/* FP:context.rs-0358 */ 
/* FP:context.rs-0359 */     fn recursion_limit(self) -> usize {
/* FP:context.rs-0360 */         self.recursion_limit().0
/* FP:context.rs-0361 */     }
/* FP:context.rs-0362 */ 
/* FP:context.rs-0363 */     type Features = &'tcx crate::rustc_feature::Features;
/* FP:context.rs-0364 */ 
/* FP:context.rs-0365 */     fn features(self) -> Self::Features {
/* FP:context.rs-0366 */         self.features()
/* FP:context.rs-0367 */     }
/* FP:context.rs-0368 */ 
/* FP:context.rs-0369 */     fn coroutine_hidden_types(
/* FP:context.rs-0370 */         self,
/* FP:context.rs-0371 */         def_id: DefId,
/* FP:context.rs-0372 */     ) -> ty::EarlyBinder<'tcx, ty::Binder<'tcx, ty::CoroutineWitnessTypes<TyCtxt<'tcx>>>> {
/* FP:context.rs-0373 */         self.coroutine_hidden_types(def_id)
/* FP:context.rs-0374 */     }
/* FP:context.rs-0375 */ 
/* FP:context.rs-0376 */     fn fn_sig(self, def_id: DefId) -> ty::EarlyBinder<'tcx, ty::PolyFnSig<'tcx>> {
/* FP:context.rs-0377 */         self.fn_sig(def_id)
/* FP:context.rs-0378 */     }
/* FP:context.rs-0379 */ 
/* FP:context.rs-0380 */     fn coroutine_movability(self, def_id: DefId) -> crate::rustc_ast::Movability {
/* FP:context.rs-0381 */         self.coroutine_movability(def_id)
/* FP:context.rs-0382 */     }
/* FP:context.rs-0383 */ 
/* FP:context.rs-0384 */     fn coroutine_for_closure(self, def_id: DefId) -> DefId {
/* FP:context.rs-0385 */         self.coroutine_for_closure(def_id)
/* FP:context.rs-0386 */     }
/* FP:context.rs-0387 */ 
/* FP:context.rs-0388 */     fn generics_require_sized_self(self, def_id: DefId) -> bool {
/* FP:context.rs-0389 */         self.generics_require_sized_self(def_id)
/* FP:context.rs-0390 */     }
/* FP:context.rs-0391 */ 
/* FP:context.rs-0392 */     fn item_bounds(
/* FP:context.rs-0393 */         self,
/* FP:context.rs-0394 */         def_id: DefId,
/* FP:context.rs-0395 */     ) -> ty::EarlyBinder<'tcx, impl IntoIterator<Item = ty::Clause<'tcx>>> {
/* FP:context.rs-0396 */         self.item_bounds(def_id).map_bound(IntoIterator::into_iter)
/* FP:context.rs-0397 */     }
/* FP:context.rs-0398 */ 
/* FP:context.rs-0399 */     fn item_self_bounds(
/* FP:context.rs-0400 */         self,
/* FP:context.rs-0401 */         def_id: DefId,
/* FP:context.rs-0402 */     ) -> ty::EarlyBinder<'tcx, impl IntoIterator<Item = ty::Clause<'tcx>>> {
/* FP:context.rs-0403 */         self.item_self_bounds(def_id).map_bound(IntoIterator::into_iter)
/* FP:context.rs-0404 */     }
/* FP:context.rs-0405 */ 
/* FP:context.rs-0406 */     fn item_non_self_bounds(
/* FP:context.rs-0407 */         self,
/* FP:context.rs-0408 */         def_id: DefId,
/* FP:context.rs-0409 */     ) -> ty::EarlyBinder<'tcx, impl IntoIterator<Item = ty::Clause<'tcx>>> {
/* FP:context.rs-0410 */         self.item_non_self_bounds(def_id).map_bound(IntoIterator::into_iter)
/* FP:context.rs-0411 */     }
/* FP:context.rs-0412 */ 
/* FP:context.rs-0413 */     fn predicates_of(
/* FP:context.rs-0414 */         self,
/* FP:context.rs-0415 */         def_id: DefId,
/* FP:context.rs-0416 */     ) -> ty::EarlyBinder<'tcx, impl IntoIterator<Item = ty::Clause<'tcx>>> {
/* FP:context.rs-0417 */         ty::EarlyBinder::bind(
/* FP:context.rs-0418 */             self.predicates_of(def_id).instantiate_identity(self).predicates.into_iter(),
/* FP:context.rs-0419 */         )
/* FP:context.rs-0420 */     }
/* FP:context.rs-0421 */ 
/* FP:context.rs-0422 */     fn own_predicates_of(
/* FP:context.rs-0423 */         self,
/* FP:context.rs-0424 */         def_id: DefId,
/* FP:context.rs-0425 */     ) -> ty::EarlyBinder<'tcx, impl IntoIterator<Item = ty::Clause<'tcx>>> {
/* FP:context.rs-0426 */         ty::EarlyBinder::bind(
/* FP:context.rs-0427 */             self.predicates_of(def_id).instantiate_own_identity().map(|(clause, _)| clause),
/* FP:context.rs-0428 */         )
/* FP:context.rs-0429 */     }
/* FP:context.rs-0430 */ 
/* FP:context.rs-0431 */     fn explicit_super_predicates_of(
/* FP:context.rs-0432 */         self,
/* FP:context.rs-0433 */         def_id: DefId,
/* FP:context.rs-0434 */     ) -> ty::EarlyBinder<'tcx, impl IntoIterator<Item = (ty::Clause<'tcx>, Span)>> {
/* FP:context.rs-0435 */         self.explicit_super_predicates_of(def_id).map_bound(|preds| preds.into_iter().copied())
/* FP:context.rs-0436 */     }
/* FP:context.rs-0437 */ 
/* FP:context.rs-0438 */     fn explicit_implied_predicates_of(
/* FP:context.rs-0439 */         self,
/* FP:context.rs-0440 */         def_id: DefId,
/* FP:context.rs-0441 */     ) -> ty::EarlyBinder<'tcx, impl IntoIterator<Item = (ty::Clause<'tcx>, Span)>> {
/* FP:context.rs-0442 */         self.explicit_implied_predicates_of(def_id).map_bound(|preds| preds.into_iter().copied())
/* FP:context.rs-0443 */     }
/* FP:context.rs-0444 */ 
/* FP:context.rs-0445 */     fn impl_super_outlives(
/* FP:context.rs-0446 */         self,
/* FP:context.rs-0447 */         impl_def_id: DefId,
/* FP:context.rs-0448 */     ) -> ty::EarlyBinder<'tcx, impl IntoIterator<Item = ty::Clause<'tcx>>> {
/* FP:context.rs-0449 */         self.impl_super_outlives(impl_def_id)
/* FP:context.rs-0450 */     }
/* FP:context.rs-0451 */ 
/* FP:context.rs-0452 */     fn impl_is_const(self, def_id: DefId) -> bool {
/* FP:context.rs-0453 */         debug_assert_matches!(self.def_kind(def_id), DefKind::Impl { of_trait: true });
/* FP:context.rs-0454 */         self.is_conditionally_const(def_id)
/* FP:context.rs-0455 */     }
/* FP:context.rs-0456 */ 
/* FP:context.rs-0457 */     fn fn_is_const(self, def_id: DefId) -> bool {
/* FP:context.rs-0458 */         debug_assert_matches!(
/* FP:context.rs-0459 */             self.def_kind(def_id),
/* FP:context.rs-0460 */             DefKind::Fn | DefKind::AssocFn | DefKind::Ctor(CtorOf::Struct, CtorKind::Fn)
/* FP:context.rs-0461 */         );
/* FP:context.rs-0462 */         self.is_conditionally_const(def_id)
/* FP:context.rs-0463 */     }
/* FP:context.rs-0464 */ 
/* FP:context.rs-0465 */     fn alias_has_const_conditions(self, def_id: DefId) -> bool {
/* FP:context.rs-0466 */         debug_assert_matches!(self.def_kind(def_id), DefKind::AssocTy | DefKind::OpaqueTy);
/* FP:context.rs-0467 */         self.is_conditionally_const(def_id)
/* FP:context.rs-0468 */     }
/* FP:context.rs-0469 */ 
/* FP:context.rs-0470 */     fn const_conditions(
/* FP:context.rs-0471 */         self,
/* FP:context.rs-0472 */         def_id: DefId,
/* FP:context.rs-0473 */     ) -> ty::EarlyBinder<'tcx, impl IntoIterator<Item = ty::Binder<'tcx, ty::TraitRef<'tcx>>>> {
/* FP:context.rs-0474 */         ty::EarlyBinder::bind(
/* FP:context.rs-0475 */             self.const_conditions(def_id).instantiate_identity(self).into_iter().map(|(c, _)| c),
/* FP:context.rs-0476 */         )
/* FP:context.rs-0477 */     }
/* FP:context.rs-0478 */ 
/* FP:context.rs-0479 */     fn explicit_implied_const_bounds(
/* FP:context.rs-0480 */         self,
/* FP:context.rs-0481 */         def_id: DefId,
/* FP:context.rs-0482 */     ) -> ty::EarlyBinder<'tcx, impl IntoIterator<Item = ty::Binder<'tcx, ty::TraitRef<'tcx>>>> {
/* FP:context.rs-0483 */         ty::EarlyBinder::bind(
/* FP:context.rs-0484 */             self.explicit_implied_const_bounds(def_id).iter_identity_copied().map(|(c, _)| c),
/* FP:context.rs-0485 */         )
/* FP:context.rs-0486 */     }
/* FP:context.rs-0487 */ 
/* FP:context.rs-0488 */     fn impl_self_is_guaranteed_unsized(self, impl_def_id: DefId) -> bool {
/* FP:context.rs-0489 */         self.impl_self_is_guaranteed_unsized(impl_def_id)
/* FP:context.rs-0490 */     }
/* FP:context.rs-0491 */ 
/* FP:context.rs-0492 */     fn has_target_features(self, def_id: DefId) -> bool {
/* FP:context.rs-0493 */         !self.codegen_fn_attrs(def_id).target_features.is_empty()
/* FP:context.rs-0494 */     }
/* FP:context.rs-0495 */ 
/* FP:context.rs-0496 */     fn require_lang_item(self, lang_item: SolverLangItem) -> DefId {
/* FP:context.rs-0497 */         self.require_lang_item(solver_lang_item_to_lang_item(lang_item), DUMMY_SP)
/* FP:context.rs-0498 */     }
/* FP:context.rs-0499 */ 
/* FP:context.rs-0500 */     fn require_trait_lang_item(self, lang_item: SolverTraitLangItem) -> DefId {
/* FP:context.rs-0501 */         self.require_lang_item(solver_trait_lang_item_to_lang_item(lang_item), DUMMY_SP)
/* FP:context.rs-0502 */     }
/* FP:context.rs-0503 */ 
/* FP:context.rs-0504 */     fn require_adt_lang_item(self, lang_item: SolverAdtLangItem) -> DefId {
/* FP:context.rs-0505 */         self.require_lang_item(solver_adt_lang_item_to_lang_item(lang_item), DUMMY_SP)
/* FP:context.rs-0506 */     }
/* FP:context.rs-0507 */ 
/* FP:context.rs-0508 */     fn is_lang_item(self, def_id: DefId, lang_item: SolverLangItem) -> bool {
/* FP:context.rs-0509 */         self.is_lang_item(def_id, solver_lang_item_to_lang_item(lang_item))
/* FP:context.rs-0510 */     }
/* FP:context.rs-0511 */ 
/* FP:context.rs-0512 */     fn is_trait_lang_item(self, def_id: DefId, lang_item: SolverTraitLangItem) -> bool {
/* FP:context.rs-0513 */         self.is_lang_item(def_id, solver_trait_lang_item_to_lang_item(lang_item))
/* FP:context.rs-0514 */     }
/* FP:context.rs-0515 */ 
/* FP:context.rs-0516 */     fn is_adt_lang_item(self, def_id: DefId, lang_item: SolverAdtLangItem) -> bool {
/* FP:context.rs-0517 */         self.is_lang_item(def_id, solver_adt_lang_item_to_lang_item(lang_item))
/* FP:context.rs-0518 */     }
/* FP:context.rs-0519 */ 
/* FP:context.rs-0520 */     fn is_default_trait(self, def_id: DefId) -> bool {
/* FP:context.rs-0521 */         self.is_default_trait(def_id)
/* FP:context.rs-0522 */     }
/* FP:context.rs-0523 */ 
/* FP:context.rs-0524 */     fn as_lang_item(self, def_id: DefId) -> Option<SolverLangItem> {
/* FP:context.rs-0525 */         lang_item_to_solver_lang_item(self.lang_items().from_def_id(def_id)?)
/* FP:context.rs-0526 */     }
/* FP:context.rs-0527 */ 
/* FP:context.rs-0528 */     fn as_trait_lang_item(self, def_id: DefId) -> Option<SolverTraitLangItem> {
/* FP:context.rs-0529 */         lang_item_to_solver_trait_lang_item(self.lang_items().from_def_id(def_id)?)
/* FP:context.rs-0530 */     }
/* FP:context.rs-0531 */ 
/* FP:context.rs-0532 */     fn as_adt_lang_item(self, def_id: DefId) -> Option<SolverAdtLangItem> {
/* FP:context.rs-0533 */         lang_item_to_solver_adt_lang_item(self.lang_items().from_def_id(def_id)?)
/* FP:context.rs-0534 */     }
/* FP:context.rs-0535 */ 
/* FP:context.rs-0536 */     fn associated_type_def_ids(self, def_id: DefId) -> impl IntoIterator<Item = DefId> {
/* FP:context.rs-0537 */         self.associated_items(def_id)
/* FP:context.rs-0538 */             .in_definition_order()
/* FP:context.rs-0539 */             .filter(|assoc_item| assoc_item.is_type())
/* FP:context.rs-0540 */             .map(|assoc_item| assoc_item.def_id)
/* FP:context.rs-0541 */     }
/* FP:context.rs-0542 */ 
/* FP:context.rs-0543 */     // This implementation is a bit different from `TyCtxt::for_each_relevant_impl`,
/* FP:context.rs-0544 */     // since we want to skip over blanket impls for non-rigid aliases, and also we
/* FP:context.rs-0545 */     // only want to consider types that *actually* unify with float/int vars.
/* FP:context.rs-0546 */     fn for_each_relevant_impl(
/* FP:context.rs-0547 */         self,
/* FP:context.rs-0548 */         trait_def_id: DefId,
/* FP:context.rs-0549 */         self_ty: Ty<'tcx>,
/* FP:context.rs-0550 */         mut f: impl FnMut(DefId),
/* FP:context.rs-0551 */     ) {
/* FP:context.rs-0552 */         let tcx = self;
/* FP:context.rs-0553 */         let trait_impls = tcx.trait_impls_of(trait_def_id);
/* FP:context.rs-0554 */         let mut consider_impls_for_simplified_type = |simp| {
/* FP:context.rs-0555 */             if let Some(impls_for_type) = trait_impls.non_blanket_impls().get(&simp) {
/* FP:context.rs-0556 */                 for &impl_def_id in impls_for_type {
/* FP:context.rs-0557 */                     f(impl_def_id);
/* FP:context.rs-0558 */                 }
/* FP:context.rs-0559 */             }
/* FP:context.rs-0560 */         };
/* FP:context.rs-0561 */ 
/* FP:context.rs-0562 */         match self_ty.kind() {
/* FP:context.rs-0563 */             ty::Bool
/* FP:context.rs-0564 */             | ty::Char
/* FP:context.rs-0565 */             | ty::Int(_)
/* FP:context.rs-0566 */             | ty::Uint(_)
/* FP:context.rs-0567 */             | ty::Float(_)
/* FP:context.rs-0568 */             | ty::Adt(_, _)
/* FP:context.rs-0569 */             | ty::Foreign(_)
/* FP:context.rs-0570 */             | ty::Str
/* FP:context.rs-0571 */             | ty::Array(_, _)
/* FP:context.rs-0572 */             | ty::Pat(_, _)
/* FP:context.rs-0573 */             | ty::Slice(_)
/* FP:context.rs-0574 */             | ty::RawPtr(_, _)
/* FP:context.rs-0575 */             | ty::Ref(_, _, _)
/* FP:context.rs-0576 */             | ty::FnDef(_, _)
/* FP:context.rs-0577 */             | ty::FnPtr(..)
/* FP:context.rs-0578 */             | ty::Dynamic(_, _, _)
/* FP:context.rs-0579 */             | ty::Closure(..)
/* FP:context.rs-0580 */             | ty::CoroutineClosure(..)
/* FP:context.rs-0581 */             | ty::Coroutine(_, _)
/* FP:context.rs-0582 */             | ty::Never
/* FP:context.rs-0583 */             | ty::Tuple(_)
/* FP:context.rs-0584 */             | ty::UnsafeBinder(_) => {
/* FP:context.rs-0585 */                 let simp = ty::fast_reject::simplify_type(
/* FP:context.rs-0586 */                     tcx,
/* FP:context.rs-0587 */                     self_ty,
/* FP:context.rs-0588 */                     ty::fast_reject::TreatParams::AsRigid,
/* FP:context.rs-0589 */                 )
/* FP:context.rs-0590 */                 .unwrap();
/* FP:context.rs-0591 */                 consider_impls_for_simplified_type(simp);
/* FP:context.rs-0592 */             }
/* FP:context.rs-0593 */ 
/* FP:context.rs-0594 */             // HACK: For integer and float variables we have to manually look at all impls
/* FP:context.rs-0595 */             // which have some integer or float as a self type.
/* FP:context.rs-0596 */             ty::Infer(ty::IntVar(_)) => {
/* FP:context.rs-0597 */                 use ty::IntTy::*;
/* FP:context.rs-0598 */                 use ty::UintTy::*;
/* FP:context.rs-0599 */                 // This causes a compiler error if any new integer kinds are added.
/* FP:context.rs-0600 */                 let (I8 | I16 | I32 | I64 | I128 | Isize): ty::IntTy;
/* FP:context.rs-0601 */                 let (U8 | U16 | U32 | U64 | U128 | Usize): ty::UintTy;
/* FP:context.rs-0602 */                 let possible_integers = [
/* FP:context.rs-0603 */                     // signed integers
/* FP:context.rs-0604 */                     ty::SimplifiedType::Int(I8),
/* FP:context.rs-0605 */                     ty::SimplifiedType::Int(I16),
/* FP:context.rs-0606 */                     ty::SimplifiedType::Int(I32),
/* FP:context.rs-0607 */                     ty::SimplifiedType::Int(I64),
/* FP:context.rs-0608 */                     ty::SimplifiedType::Int(I128),
/* FP:context.rs-0609 */                     ty::SimplifiedType::Int(Isize),
/* FP:context.rs-0610 */                     // unsigned integers
/* FP:context.rs-0611 */                     ty::SimplifiedType::Uint(U8),
/* FP:context.rs-0612 */                     ty::SimplifiedType::Uint(U16),
/* FP:context.rs-0613 */                     ty::SimplifiedType::Uint(U32),
/* FP:context.rs-0614 */                     ty::SimplifiedType::Uint(U64),
/* FP:context.rs-0615 */                     ty::SimplifiedType::Uint(U128),
/* FP:context.rs-0616 */                     ty::SimplifiedType::Uint(Usize),
/* FP:context.rs-0617 */                 ];
/* FP:context.rs-0618 */                 for simp in possible_integers {
/* FP:context.rs-0619 */                     consider_impls_for_simplified_type(simp);
/* FP:context.rs-0620 */                 }
/* FP:context.rs-0621 */             }
/* FP:context.rs-0622 */ 
/* FP:context.rs-0623 */             ty::Infer(ty::FloatVar(_)) => {
/* FP:context.rs-0624 */                 // This causes a compiler error if any new float kinds are added.
/* FP:context.rs-0625 */                 let (ty::FloatTy::F16 | ty::FloatTy::F32 | ty::FloatTy::F64 | ty::FloatTy::F128);
/* FP:context.rs-0626 */                 let possible_floats = [
/* FP:context.rs-0627 */                     ty::SimplifiedType::Float(ty::FloatTy::F16),
/* FP:context.rs-0628 */                     ty::SimplifiedType::Float(ty::FloatTy::F32),
/* FP:context.rs-0629 */                     ty::SimplifiedType::Float(ty::FloatTy::F64),
/* FP:context.rs-0630 */                     ty::SimplifiedType::Float(ty::FloatTy::F128),
/* FP:context.rs-0631 */                 ];
/* FP:context.rs-0632 */ 
/* FP:context.rs-0633 */                 for simp in possible_floats {
/* FP:context.rs-0634 */                     consider_impls_for_simplified_type(simp);
/* FP:context.rs-0635 */                 }
/* FP:context.rs-0636 */             }
/* FP:context.rs-0637 */ 
/* FP:context.rs-0638 */             // The only traits applying to aliases and placeholders are blanket impls.
/* FP:context.rs-0639 */             //
/* FP:context.rs-0640 */             // Impls which apply to an alias after normalization are handled by
/* FP:context.rs-0641 */             // `assemble_candidates_after_normalizing_self_ty`.
/* FP:context.rs-0642 */             ty::Alias(_, _) | ty::Placeholder(..) | ty::Error(_) => (),
/* FP:context.rs-0643 */ 
/* FP:context.rs-0644 */             // FIXME: These should ideally not exist as a self type. It would be nice for
/* FP:context.rs-0645 */             // the builtin auto trait impls of coroutines to instead directly recurse
/* FP:context.rs-0646 */             // into the witness.
/* FP:context.rs-0647 */             ty::CoroutineWitness(..) => (),
/* FP:context.rs-0648 */ 
/* FP:context.rs-0649 */             // These variants should not exist as a self type.
/* FP:context.rs-0650 */             ty::Infer(ty::TyVar(_) | ty::FreshTy(_) | ty::FreshIntTy(_) | ty::FreshFloatTy(_))
/* FP:context.rs-0651 */             | ty::Param(_)
/* FP:context.rs-0652 */             | ty::Bound(_, _) => bug!("unexpected self type: {self_ty}"),
/* FP:context.rs-0653 */         }
/* FP:context.rs-0654 */ 
/* FP:context.rs-0655 */         #[allow(rustc::usage_of_type_ir_traits)]
/* FP:context.rs-0656 */         self.for_each_blanket_impl(trait_def_id, f)
/* FP:context.rs-0657 */     }
/* FP:context.rs-0658 */     fn for_each_blanket_impl(self, trait_def_id: DefId, mut f: impl FnMut(DefId)) {
/* FP:context.rs-0659 */         let trait_impls = self.trait_impls_of(trait_def_id);
/* FP:context.rs-0660 */         for &impl_def_id in trait_impls.blanket_impls() {
/* FP:context.rs-0661 */             f(impl_def_id);
/* FP:context.rs-0662 */         }
/* FP:context.rs-0663 */     }
/* FP:context.rs-0664 */ 
/* FP:context.rs-0665 */     fn has_item_definition(self, def_id: DefId) -> bool {
/* FP:context.rs-0666 */         self.defaultness(def_id).has_value()
/* FP:context.rs-0667 */     }
/* FP:context.rs-0668 */ 
/* FP:context.rs-0669 */     fn impl_specializes(self, impl_def_id: Self::DefId, victim_def_id: Self::DefId) -> bool {
/* FP:context.rs-0670 */         self.specializes((impl_def_id, victim_def_id))
/* FP:context.rs-0671 */     }
/* FP:context.rs-0672 */ 
/* FP:context.rs-0673 */     fn impl_is_default(self, impl_def_id: DefId) -> bool {
/* FP:context.rs-0674 */         self.defaultness(impl_def_id).is_default()
/* FP:context.rs-0675 */     }
/* FP:context.rs-0676 */ 
/* FP:context.rs-0677 */     fn impl_trait_ref(self, impl_def_id: DefId) -> ty::EarlyBinder<'tcx, ty::TraitRef<'tcx>> {
/* FP:context.rs-0678 */         self.impl_trait_ref(impl_def_id).unwrap()
/* FP:context.rs-0679 */     }
/* FP:context.rs-0680 */ 
/* FP:context.rs-0681 */     fn impl_polarity(self, impl_def_id: DefId) -> ty::ImplPolarity {
/* FP:context.rs-0682 */         self.impl_polarity(impl_def_id)
/* FP:context.rs-0683 */     }
/* FP:context.rs-0684 */ 
/* FP:context.rs-0685 */     fn trait_is_auto(self, trait_def_id: DefId) -> bool {
/* FP:context.rs-0686 */         self.trait_is_auto(trait_def_id)
/* FP:context.rs-0687 */     }
/* FP:context.rs-0688 */ 
/* FP:context.rs-0689 */     fn trait_is_coinductive(self, trait_def_id: DefId) -> bool {
/* FP:context.rs-0690 */         self.trait_is_coinductive(trait_def_id)
/* FP:context.rs-0691 */     }
/* FP:context.rs-0692 */ 
/* FP:context.rs-0693 */     fn trait_is_alias(self, trait_def_id: DefId) -> bool {
/* FP:context.rs-0694 */         self.trait_is_alias(trait_def_id)
/* FP:context.rs-0695 */     }
/* FP:context.rs-0696 */ 
/* FP:context.rs-0697 */     fn trait_is_dyn_compatible(self, trait_def_id: DefId) -> bool {
/* FP:context.rs-0698 */         self.is_dyn_compatible(trait_def_id)
/* FP:context.rs-0699 */     }
/* FP:context.rs-0700 */ 
/* FP:context.rs-0701 */     fn trait_is_fundamental(self, def_id: DefId) -> bool {
/* FP:context.rs-0702 */         self.trait_def(def_id).is_fundamental
/* FP:context.rs-0703 */     }
/* FP:context.rs-0704 */ 
/* FP:context.rs-0705 */     fn trait_may_be_implemented_via_object(self, trait_def_id: DefId) -> bool {
/* FP:context.rs-0706 */         self.trait_def(trait_def_id).implement_via_object
/* FP:context.rs-0707 */     }
/* FP:context.rs-0708 */ 
/* FP:context.rs-0709 */     fn trait_is_unsafe(self, trait_def_id: Self::DefId) -> bool {
/* FP:context.rs-0710 */         self.trait_def(trait_def_id).safety.is_unsafe()
/* FP:context.rs-0711 */     }
/* FP:context.rs-0712 */ 
/* FP:context.rs-0713 */     fn is_impl_trait_in_trait(self, def_id: DefId) -> bool {
/* FP:context.rs-0714 */         self.is_impl_trait_in_trait(def_id)
/* FP:context.rs-0715 */     }
/* FP:context.rs-0716 */ 
/* FP:context.rs-0717 */     fn delay_bug(self, msg: impl ToString) -> ErrorGuaranteed {
/* FP:context.rs-0718 */         self.dcx().span_delayed_bug(DUMMY_SP, msg.to_string())
/* FP:context.rs-0719 */     }
/* FP:context.rs-0720 */ 
/* FP:context.rs-0721 */     fn is_general_coroutine(self, coroutine_def_id: DefId) -> bool {
/* FP:context.rs-0722 */         self.is_general_coroutine(coroutine_def_id)
/* FP:context.rs-0723 */     }
/* FP:context.rs-0724 */ 
/* FP:context.rs-0725 */     fn coroutine_is_async(self, coroutine_def_id: DefId) -> bool {
/* FP:context.rs-0726 */         self.coroutine_is_async(coroutine_def_id)
/* FP:context.rs-0727 */     }
/* FP:context.rs-0728 */ 
/* FP:context.rs-0729 */     fn coroutine_is_gen(self, coroutine_def_id: DefId) -> bool {
/* FP:context.rs-0730 */         self.coroutine_is_gen(coroutine_def_id)
/* FP:context.rs-0731 */     }
/* FP:context.rs-0732 */ 
/* FP:context.rs-0733 */     fn coroutine_is_async_gen(self, coroutine_def_id: DefId) -> bool {
/* FP:context.rs-0734 */         self.coroutine_is_async_gen(coroutine_def_id)
/* FP:context.rs-0735 */     }
/* FP:context.rs-0736 */ 
/* FP:context.rs-0737 */     type UnsizingParams = &'tcx crate::rustc_index::bit_set::DenseBitSet<u32>;
/* FP:context.rs-0738 */     fn unsizing_params_for_adt(self, adt_def_id: DefId) -> Self::UnsizingParams {
/* FP:context.rs-0739 */         self.unsizing_params_for_adt(adt_def_id)
/* FP:context.rs-0740 */     }
/* FP:context.rs-0741 */ 
/* FP:context.rs-0742 */     fn anonymize_bound_vars<T: TypeFoldable<TyCtxt<'tcx>>>(
/* FP:context.rs-0743 */         self,
/* FP:context.rs-0744 */         binder: ty::Binder<'tcx, T>,
/* FP:context.rs-0745 */     ) -> ty::Binder<'tcx, T> {
/* FP:context.rs-0746 */         self.anonymize_bound_vars(binder)
/* FP:context.rs-0747 */     }
/* FP:context.rs-0748 */ 
/* FP:context.rs-0749 */     fn opaque_types_defined_by(self, defining_anchor: LocalDefId) -> Self::LocalDefIds {
/* FP:context.rs-0750 */         self.opaque_types_defined_by(defining_anchor)
/* FP:context.rs-0751 */     }
/* FP:context.rs-0752 */ 
/* FP:context.rs-0753 */     fn opaque_types_and_coroutines_defined_by(
/* FP:context.rs-0754 */         self,
/* FP:context.rs-0755 */         defining_anchor: Self::LocalDefId,
/* FP:context.rs-0756 */     ) -> Self::LocalDefIds {
/* FP:context.rs-0757 */         let coroutines_defined_by = self
/* FP:context.rs-0758 */             .nested_bodies_within(defining_anchor)
/* FP:context.rs-0759 */             .iter()
/* FP:context.rs-0760 */             .filter(|def_id| self.is_coroutine(def_id.to_def_id()));
/* FP:context.rs-0761 */         self.mk_local_def_ids_from_iter(
/* FP:context.rs-0762 */             self.opaque_types_defined_by(defining_anchor).iter().chain(coroutines_defined_by),
/* FP:context.rs-0763 */         )
/* FP:context.rs-0764 */     }
/* FP:context.rs-0765 */ 
/* FP:context.rs-0766 */     type Probe = &'tcx inspect::Probe<TyCtxt<'tcx>>;
/* FP:context.rs-0767 */     fn mk_probe(self, probe: inspect::Probe<Self>) -> &'tcx inspect::Probe<TyCtxt<'tcx>> {
/* FP:context.rs-0768 */         self.arena.alloc(probe)
/* FP:context.rs-0769 */     }
/* FP:context.rs-0770 */     fn evaluate_root_goal_for_proof_tree_raw(
/* FP:context.rs-0771 */         self,
/* FP:context.rs-0772 */         canonical_goal: CanonicalInput<'tcx>,
/* FP:context.rs-0773 */     ) -> (QueryResult<'tcx>, &'tcx inspect::Probe<TyCtxt<'tcx>>) {
/* FP:context.rs-0774 */         self.evaluate_root_goal_for_proof_tree_raw(canonical_goal)
/* FP:context.rs-0775 */     }
/* FP:context.rs-0776 */ }
/* FP:context.rs-0777 */ 
/* FP:context.rs-0778 */ macro_rules! bidirectional_lang_item_map {
/* FP:context.rs-0779 */     (
/* FP:context.rs-0780 */         $solver_ty:ident, $to_solver:ident, $from_solver:ident;
/* FP:context.rs-0781 */         $($name:ident),+ $(,)?
/* FP:context.rs-0782 */     ) => {
/* FP:context.rs-0783 */         fn $from_solver(lang_item: $solver_ty) -> LangItem {
/* FP:context.rs-0784 */             match lang_item {
/* FP:context.rs-0785 */                 $($solver_ty::$name => LangItem::$name,)+
/* FP:context.rs-0786 */             }
/* FP:context.rs-0787 */         }
/* FP:context.rs-0788 */ 
/* FP:context.rs-0789 */         fn $to_solver(lang_item: LangItem) -> Option<$solver_ty> {
/* FP:context.rs-0790 */             Some(match lang_item {
/* FP:context.rs-0791 */                 $(LangItem::$name => $solver_ty::$name,)+
/* FP:context.rs-0792 */                 _ => return None,
/* FP:context.rs-0793 */             })
/* FP:context.rs-0794 */         }
/* FP:context.rs-0795 */     }
/* FP:context.rs-0796 */ }
/* FP:context.rs-0797 */ 
/* FP:context.rs-0798 */ bidirectional_lang_item_map! {
/* FP:context.rs-0799 */     SolverLangItem, lang_item_to_solver_lang_item, solver_lang_item_to_lang_item;
/* FP:context.rs-0800 */ 
/* FP:context.rs-0801 */ // tidy-alphabetical-start
/* FP:context.rs-0802 */     AsyncFnKindUpvars,
/* FP:context.rs-0803 */     AsyncFnOnceOutput,
/* FP:context.rs-0804 */     CallOnceFuture,
/* FP:context.rs-0805 */     CallRefFuture,
/* FP:context.rs-0806 */     CoroutineReturn,
/* FP:context.rs-0807 */     CoroutineYield,
/* FP:context.rs-0808 */     DynMetadata,
/* FP:context.rs-0809 */     FutureOutput,
/* FP:context.rs-0810 */     Metadata,
/* FP:context.rs-0811 */ // tidy-alphabetical-end
/* FP:context.rs-0812 */ }
/* FP:context.rs-0813 */ 
/* FP:context.rs-0814 */ bidirectional_lang_item_map! {
/* FP:context.rs-0815 */     SolverAdtLangItem, lang_item_to_solver_adt_lang_item, solver_adt_lang_item_to_lang_item;
/* FP:context.rs-0816 */ 
/* FP:context.rs-0817 */ // tidy-alphabetical-start
/* FP:context.rs-0818 */     Option,
/* FP:context.rs-0819 */     Poll,
/* FP:context.rs-0820 */ // tidy-alphabetical-end
/* FP:context.rs-0821 */ }
/* FP:context.rs-0822 */ 
/* FP:context.rs-0823 */ bidirectional_lang_item_map! {
/* FP:context.rs-0824 */     SolverTraitLangItem, lang_item_to_solver_trait_lang_item, solver_trait_lang_item_to_lang_item;
/* FP:context.rs-0825 */ 
/* FP:context.rs-0826 */ // tidy-alphabetical-start
/* FP:context.rs-0827 */     AsyncFn,
/* FP:context.rs-0828 */     AsyncFnKindHelper,
/* FP:context.rs-0829 */     AsyncFnMut,
/* FP:context.rs-0830 */     AsyncFnOnce,
/* FP:context.rs-0831 */     AsyncFnOnceOutput,
/* FP:context.rs-0832 */     AsyncIterator,
/* FP:context.rs-0833 */     BikeshedGuaranteedNoDrop,
/* FP:context.rs-0834 */     Clone,
/* FP:context.rs-0835 */     Copy,
/* FP:context.rs-0836 */     Coroutine,
/* FP:context.rs-0837 */     Destruct,
/* FP:context.rs-0838 */     DiscriminantKind,
/* FP:context.rs-0839 */     Drop,
/* FP:context.rs-0840 */     Fn,
/* FP:context.rs-0841 */     FnMut,
/* FP:context.rs-0842 */     FnOnce,
/* FP:context.rs-0843 */     FnPtrTrait,
/* FP:context.rs-0844 */     FusedIterator,
/* FP:context.rs-0845 */     Future,
/* FP:context.rs-0846 */     Iterator,
/* FP:context.rs-0847 */     MetaSized,
/* FP:context.rs-0848 */     PointeeSized,
/* FP:context.rs-0849 */     PointeeTrait,
/* FP:context.rs-0850 */     Sized,
/* FP:context.rs-0851 */     TransmuteTrait,
/* FP:context.rs-0852 */     Tuple,
/* FP:context.rs-0853 */     Unpin,
/* FP:context.rs-0854 */     Unsize,
/* FP:context.rs-0855 */ // tidy-alphabetical-end
/* FP:context.rs-0856 */ }
/* FP:context.rs-0857 */ 
/* FP:context.rs-0858 */ impl<'tcx> rustc_type_ir::inherent::DefId<TyCtxt<'tcx>> for DefId {
/* FP:context.rs-0859 */     fn is_local(self) -> bool {
/* FP:context.rs-0860 */         self.is_local()
/* FP:context.rs-0861 */     }
/* FP:context.rs-0862 */ 
/* FP:context.rs-0863 */     fn as_local(self) -> Option<LocalDefId> {
/* FP:context.rs-0864 */         self.as_local()
/* FP:context.rs-0865 */     }
/* FP:context.rs-0866 */ }
/* FP:context.rs-0867 */ 
/* FP:context.rs-0868 */ impl<'tcx> rustc_type_ir::inherent::Abi<TyCtxt<'tcx>> for ExternAbi {
/* FP:context.rs-0869 */     fn rust() -> Self {
/* FP:context.rs-0870 */         ExternAbi::Rust
/* FP:context.rs-0871 */     }
/* FP:context.rs-0872 */ 
/* FP:context.rs-0873 */     fn is_rust(self) -> bool {
/* FP:context.rs-0874 */         matches!(self, ExternAbi::Rust)
/* FP:context.rs-0875 */     }
/* FP:context.rs-0876 */ }
/* FP:context.rs-0877 */ 
/* FP:context.rs-0878 */ impl<'tcx> rustc_type_ir::inherent::Safety<TyCtxt<'tcx>> for hir::Safety {
/* FP:context.rs-0879 */     fn safe() -> Self {
/* FP:context.rs-0880 */         hir::Safety::Safe
/* FP:context.rs-0881 */     }
/* FP:context.rs-0882 */ 
/* FP:context.rs-0883 */     fn is_safe(self) -> bool {
/* FP:context.rs-0884 */         self.is_safe()
/* FP:context.rs-0885 */     }
/* FP:context.rs-0886 */ 
/* FP:context.rs-0887 */     fn prefix_str(self) -> &'static str {
/* FP:context.rs-0888 */         self.prefix_str()
/* FP:context.rs-0889 */     }
/* FP:context.rs-0890 */ }
/* FP:context.rs-0891 */ 
/* FP:context.rs-0892 */ impl<'tcx> rustc_type_ir::inherent::Features<TyCtxt<'tcx>> for &'tcx crate::rustc_feature::Features {
/* FP:context.rs-0893 */     fn generic_const_exprs(self) -> bool {
/* FP:context.rs-0894 */         self.generic_const_exprs()
/* FP:context.rs-0895 */     }
/* FP:context.rs-0896 */ 
/* FP:context.rs-0897 */     fn coroutine_clone(self) -> bool {
/* FP:context.rs-0898 */         self.coroutine_clone()
/* FP:context.rs-0899 */     }
/* FP:context.rs-0900 */ 
/* FP:context.rs-0901 */     fn associated_const_equality(self) -> bool {
/* FP:context.rs-0902 */         self.associated_const_equality()
/* FP:context.rs-0903 */     }
/* FP:context.rs-0904 */ 
/* FP:context.rs-0905 */     fn feature_bound_holds_in_crate(self, symbol: Symbol) -> bool {
/* FP:context.rs-0906 */         // We don't consider feature bounds to hold in the crate when `staged_api` feature is
/* FP:context.rs-0907 */         // enabled, even if it is enabled through `#[feature]`.
/* FP:context.rs-0908 */         // This is to prevent accidentally leaking unstable APIs to stable.
/* FP:context.rs-0909 */         !self.staged_api() && self.enabled(symbol)
/* FP:context.rs-0910 */     }
/* FP:context.rs-0911 */ }
/* FP:context.rs-0912 */ 
/* FP:context.rs-0913 */ impl<'tcx> rustc_type_ir::inherent::Span<TyCtxt<'tcx>> for Span {
/* FP:context.rs-0914 */     fn dummy() -> Self {
/* FP:context.rs-0915 */         DUMMY_SP
/* FP:context.rs-0916 */     }
/* FP:context.rs-0917 */ }
/* FP:context.rs-0918 */ 
/* FP:context.rs-0919 */ type InternedSet<'tcx, T> = ShardedHashMap<InternedInSet<'tcx, T>, ()>;
/* FP:context.rs-0920 */ 
/* FP:context.rs-0921 */ pub struct CtxtInterners<'tcx> {
/* FP:context.rs-0922 */     /// The arena that types, regions, etc. are allocated from.
/* FP:context.rs-0923 */     arena: &'tcx WorkerLocal<Arena<'tcx>>,
/* FP:context.rs-0924 */ 
/* FP:context.rs-0925 */     // Specifically use a speedy hash algorithm for these hash sets, since
/* FP:context.rs-0926 */     // they're accessed quite often.
/* FP:context.rs-0927 */     type_: InternedSet<'tcx, WithCachedTypeInfo<TyKind<'tcx>>>,
/* FP:context.rs-0928 */     const_lists: InternedSet<'tcx, List<ty::Const<'tcx>>>,
/* FP:context.rs-0929 */     args: InternedSet<'tcx, GenericArgs<'tcx>>,
/* FP:context.rs-0930 */     type_lists: InternedSet<'tcx, List<Ty<'tcx>>>,
/* FP:context.rs-0931 */     canonical_var_kinds: InternedSet<'tcx, List<CanonicalVarKind<'tcx>>>,
/* FP:context.rs-0932 */     region: InternedSet<'tcx, RegionKind<'tcx>>,
/* FP:context.rs-0933 */     poly_existential_predicates: InternedSet<'tcx, List<PolyExistentialPredicate<'tcx>>>,
/* FP:context.rs-0934 */     predicate: InternedSet<'tcx, WithCachedTypeInfo<ty::Binder<'tcx, PredicateKind<'tcx>>>>,
/* FP:context.rs-0935 */     clauses: InternedSet<'tcx, ListWithCachedTypeInfo<Clause<'tcx>>>,
/* FP:context.rs-0936 */     projs: InternedSet<'tcx, List<ProjectionKind>>,
/* FP:context.rs-0937 */     place_elems: InternedSet<'tcx, List<PlaceElem<'tcx>>>,
/* FP:context.rs-0938 */     const_: InternedSet<'tcx, WithCachedTypeInfo<ty::ConstKind<'tcx>>>,
/* FP:context.rs-0939 */     pat: InternedSet<'tcx, PatternKind<'tcx>>,
/* FP:context.rs-0940 */     const_allocation: InternedSet<'tcx, Allocation>,
/* FP:context.rs-0941 */     bound_variable_kinds: InternedSet<'tcx, List<ty::BoundVariableKind>>,
/* FP:context.rs-0942 */     layout: InternedSet<'tcx, LayoutData<FieldIdx, VariantIdx>>,
/* FP:context.rs-0943 */     adt_def: InternedSet<'tcx, AdtDefData>,
/* FP:context.rs-0944 */     external_constraints: InternedSet<'tcx, ExternalConstraintsData<TyCtxt<'tcx>>>,
/* FP:context.rs-0945 */     predefined_opaques_in_body: InternedSet<'tcx, PredefinedOpaquesData<TyCtxt<'tcx>>>,
/* FP:context.rs-0946 */     fields: InternedSet<'tcx, List<FieldIdx>>,
/* FP:context.rs-0947 */     local_def_ids: InternedSet<'tcx, List<LocalDefId>>,
/* FP:context.rs-0948 */     captures: InternedSet<'tcx, List<&'tcx ty::CapturedPlace<'tcx>>>,
/* FP:context.rs-0949 */     offset_of: InternedSet<'tcx, List<(VariantIdx, FieldIdx)>>,
/* FP:context.rs-0950 */     valtree: InternedSet<'tcx, ty::ValTreeKind<'tcx>>,
/* FP:context.rs-0951 */     patterns: InternedSet<'tcx, List<ty::Pattern<'tcx>>>,
/* FP:context.rs-0952 */     outlives: InternedSet<'tcx, List<ty::ArgOutlivesPredicate<'tcx>>>,
/* FP:context.rs-0953 */ }
/* FP:context.rs-0954 */ 
/* FP:context.rs-0955 */ impl<'tcx> CtxtInterners<'tcx> {
/* FP:context.rs-0956 */     fn new(arena: &'tcx WorkerLocal<Arena<'tcx>>) -> CtxtInterners<'tcx> {
/* FP:context.rs-0957 */         // Default interner size - this value has been chosen empirically, and may need to be adjusted
/* FP:context.rs-0958 */         // as the compiler evolves.
/* FP:context.rs-0959 */         const N: usize = 2048;
/* FP:context.rs-0960 */         CtxtInterners {
/* FP:context.rs-0961 */             arena,
/* FP:context.rs-0962 */             // The factors have been chosen by @FractalFir based on observed interner sizes, and local perf runs.
/* FP:context.rs-0963 */             // To get the interner sizes, insert `eprintln` printing the size of the interner in functions like `intern_ty`.
/* FP:context.rs-0964 */             // Bigger benchmarks tend to give more accurate ratios, so use something like `x perf eprintln --includes cargo`.
/* FP:context.rs-0965 */             type_: InternedSet::with_capacity(N * 16),
/* FP:context.rs-0966 */             const_lists: InternedSet::with_capacity(N * 4),
/* FP:context.rs-0967 */             args: InternedSet::with_capacity(N * 4),
/* FP:context.rs-0968 */             type_lists: InternedSet::with_capacity(N * 4),
/* FP:context.rs-0969 */             region: InternedSet::with_capacity(N * 4),
/* FP:context.rs-0970 */             poly_existential_predicates: InternedSet::with_capacity(N / 4),
/* FP:context.rs-0971 */             canonical_var_kinds: InternedSet::with_capacity(N / 2),
/* FP:context.rs-0972 */             predicate: InternedSet::with_capacity(N),
/* FP:context.rs-0973 */             clauses: InternedSet::with_capacity(N),
/* FP:context.rs-0974 */             projs: InternedSet::with_capacity(N * 4),
/* FP:context.rs-0975 */             place_elems: InternedSet::with_capacity(N * 2),
/* FP:context.rs-0976 */             const_: InternedSet::with_capacity(N * 2),
/* FP:context.rs-0977 */             pat: InternedSet::with_capacity(N),
/* FP:context.rs-0978 */             const_allocation: InternedSet::with_capacity(N),
/* FP:context.rs-0979 */             bound_variable_kinds: InternedSet::with_capacity(N * 2),
/* FP:context.rs-0980 */             layout: InternedSet::with_capacity(N),
/* FP:context.rs-0981 */             adt_def: InternedSet::with_capacity(N),
/* FP:context.rs-0982 */             external_constraints: InternedSet::with_capacity(N),
/* FP:context.rs-0983 */             predefined_opaques_in_body: InternedSet::with_capacity(N),
/* FP:context.rs-0984 */             fields: InternedSet::with_capacity(N * 4),
/* FP:context.rs-0985 */             local_def_ids: InternedSet::with_capacity(N),
/* FP:context.rs-0986 */             captures: InternedSet::with_capacity(N),
/* FP:context.rs-0987 */             offset_of: InternedSet::with_capacity(N),
/* FP:context.rs-0988 */             valtree: InternedSet::with_capacity(N),
/* FP:context.rs-0989 */             patterns: InternedSet::with_capacity(N),
/* FP:context.rs-0990 */             outlives: InternedSet::with_capacity(N),
/* FP:context.rs-0991 */         }
/* FP:context.rs-0992 */     }
/* FP:context.rs-0993 */ 
/* FP:context.rs-0994 */     /// Interns a type. (Use `mk_*` functions instead, where possible.)
/* FP:context.rs-0995 */     #[allow(rustc::usage_of_ty_tykind)]
/* FP:context.rs-0996 */     #[inline(never)]
/* FP:context.rs-0997 */     fn intern_ty(&self, kind: TyKind<'tcx>, sess: &Session, untracked: &Untracked) -> Ty<'tcx> {
/* FP:context.rs-0998 */         Ty(Interned::new_unchecked(
/* FP:context.rs-0999 */             self.type_
/* FP:context.rs-1000 */                 .intern(kind, |kind| {
/* FP:context.rs-1001 */                     let flags = ty::FlagComputation::<TyCtxt<'tcx>>::for_kind(&kind);
/* FP:context.rs-1002 */                     let stable_hash = self.stable_hash(&flags, sess, untracked, &kind);
/* FP:context.rs-1003 */ 
/* FP:context.rs-1004 */                     InternedInSet(self.arena.alloc(WithCachedTypeInfo {
/* FP:context.rs-1005 */                         internee: kind,
/* FP:context.rs-1006 */                         stable_hash,
/* FP:context.rs-1007 */                         flags: flags.flags,
/* FP:context.rs-1008 */                         outer_exclusive_binder: flags.outer_exclusive_binder,
/* FP:context.rs-1009 */                     }))
/* FP:context.rs-1010 */                 })
/* FP:context.rs-1011 */                 .0,
/* FP:context.rs-1012 */         ))
/* FP:context.rs-1013 */     }
/* FP:context.rs-1014 */ 
/* FP:context.rs-1015 */     /// Interns a const. (Use `mk_*` functions instead, where possible.)
/* FP:context.rs-1016 */     #[allow(rustc::usage_of_ty_tykind)]
/* FP:context.rs-1017 */     #[inline(never)]
/* FP:context.rs-1018 */     fn intern_const(
/* FP:context.rs-1019 */         &self,
/* FP:context.rs-1020 */         kind: ty::ConstKind<'tcx>,
/* FP:context.rs-1021 */         sess: &Session,
/* FP:context.rs-1022 */         untracked: &Untracked,
/* FP:context.rs-1023 */     ) -> Const<'tcx> {
/* FP:context.rs-1024 */         Const(Interned::new_unchecked(
/* FP:context.rs-1025 */             self.const_
/* FP:context.rs-1026 */                 .intern(kind, |kind: ty::ConstKind<'_>| {
/* FP:context.rs-1027 */                     let flags = ty::FlagComputation::<TyCtxt<'tcx>>::for_const_kind(&kind);
/* FP:context.rs-1028 */                     let stable_hash = self.stable_hash(&flags, sess, untracked, &kind);
/* FP:context.rs-1029 */ 
/* FP:context.rs-1030 */                     InternedInSet(self.arena.alloc(WithCachedTypeInfo {
/* FP:context.rs-1031 */                         internee: kind,
/* FP:context.rs-1032 */                         stable_hash,
/* FP:context.rs-1033 */                         flags: flags.flags,
/* FP:context.rs-1034 */                         outer_exclusive_binder: flags.outer_exclusive_binder,
/* FP:context.rs-1035 */                     }))
/* FP:context.rs-1036 */                 })
/* FP:context.rs-1037 */                 .0,
/* FP:context.rs-1038 */         ))
/* FP:context.rs-1039 */     }
/* FP:context.rs-1040 */ 
/* FP:context.rs-1041 */     fn stable_hash<'a, T: HashStable<StableHashingContext<'a>>>(
/* FP:context.rs-1042 */         &self,
/* FP:context.rs-1043 */         flags: &ty::FlagComputation<TyCtxt<'tcx>>,
/* FP:context.rs-1044 */         sess: &'a Session,
/* FP:context.rs-1045 */         untracked: &'a Untracked,
/* FP:context.rs-1046 */         val: &T,
/* FP:context.rs-1047 */     ) -> Fingerprint {
/* FP:context.rs-1048 */         // It's impossible to hash inference variables (and will ICE), so we don't need to try to cache them.
/* FP:context.rs-1049 */         // Without incremental, we rarely stable-hash types, so let's not do it proactively.
/* FP:context.rs-1050 */         if flags.flags.intersects(TypeFlags::HAS_INFER) || sess.opts.incremental.is_none() {
/* FP:context.rs-1051 */             Fingerprint::ZERO
/* FP:context.rs-1052 */         } else {
/* FP:context.rs-1053 */             let mut hasher = StableHasher::new();
/* FP:context.rs-1054 */             let mut hcx = StableHashingContext::new(sess, untracked);
/* FP:context.rs-1055 */             val.hash_stable(&mut hcx, &mut hasher);
/* FP:context.rs-1056 */             hasher.finish()
/* FP:context.rs-1057 */         }
/* FP:context.rs-1058 */     }
/* FP:context.rs-1059 */ 
/* FP:context.rs-1060 */     /// Interns a predicate. (Use `mk_predicate` instead, where possible.)
/* FP:context.rs-1061 */     #[inline(never)]
/* FP:context.rs-1062 */     fn intern_predicate(
/* FP:context.rs-1063 */         &self,
/* FP:context.rs-1064 */         kind: Binder<'tcx, PredicateKind<'tcx>>,
/* FP:context.rs-1065 */         sess: &Session,
/* FP:context.rs-1066 */         untracked: &Untracked,
/* FP:context.rs-1067 */     ) -> Predicate<'tcx> {
/* FP:context.rs-1068 */         Predicate(Interned::new_unchecked(
/* FP:context.rs-1069 */             self.predicate
/* FP:context.rs-1070 */                 .intern(kind, |kind| {
/* FP:context.rs-1071 */                     let flags = ty::FlagComputation::<TyCtxt<'tcx>>::for_predicate(kind);
/* FP:context.rs-1072 */ 
/* FP:context.rs-1073 */                     let stable_hash = self.stable_hash(&flags, sess, untracked, &kind);
/* FP:context.rs-1074 */ 
/* FP:context.rs-1075 */                     InternedInSet(self.arena.alloc(WithCachedTypeInfo {
/* FP:context.rs-1076 */                         internee: kind,
/* FP:context.rs-1077 */                         stable_hash,
/* FP:context.rs-1078 */                         flags: flags.flags,
/* FP:context.rs-1079 */                         outer_exclusive_binder: flags.outer_exclusive_binder,
/* FP:context.rs-1080 */                     }))
/* FP:context.rs-1081 */                 })
/* FP:context.rs-1082 */                 .0,
/* FP:context.rs-1083 */         ))
/* FP:context.rs-1084 */     }
/* FP:context.rs-1085 */ 
/* FP:context.rs-1086 */     fn intern_clauses(&self, clauses: &[Clause<'tcx>]) -> Clauses<'tcx> {
/* FP:context.rs-1087 */         if clauses.is_empty() {
/* FP:context.rs-1088 */             ListWithCachedTypeInfo::empty()
/* FP:context.rs-1089 */         } else {
/* FP:context.rs-1090 */             self.clauses
/* FP:context.rs-1091 */                 .intern_ref(clauses, || {
/* FP:context.rs-1092 */                     let flags = ty::FlagComputation::<TyCtxt<'tcx>>::for_clauses(clauses);
/* FP:context.rs-1093 */ 
/* FP:context.rs-1094 */                     InternedInSet(ListWithCachedTypeInfo::from_arena(
/* FP:context.rs-1095 */                         &*self.arena,
/* FP:context.rs-1096 */                         flags.into(),
/* FP:context.rs-1097 */                         clauses,
/* FP:context.rs-1098 */                     ))
/* FP:context.rs-1099 */                 })
/* FP:context.rs-1100 */                 .0
/* FP:context.rs-1101 */         }
/* FP:context.rs-1102 */     }
/* FP:context.rs-1103 */ }
/* FP:context.rs-1104 */ 
/* FP:context.rs-1105 */ // For these preinterned values, an alternative would be to have
/* FP:context.rs-1106 */ // variable-length vectors that grow as needed. But that turned out to be
/* FP:context.rs-1107 */ // slightly more complex and no faster.
/* FP:context.rs-1108 */ 
/* FP:context.rs-1109 */ const NUM_PREINTERNED_TY_VARS: u32 = 100;
/* FP:context.rs-1110 */ const NUM_PREINTERNED_FRESH_TYS: u32 = 20;
/* FP:context.rs-1111 */ const NUM_PREINTERNED_FRESH_INT_TYS: u32 = 3;
/* FP:context.rs-1112 */ const NUM_PREINTERNED_FRESH_FLOAT_TYS: u32 = 3;
/* FP:context.rs-1113 */ const NUM_PREINTERNED_ANON_BOUND_TYS_I: u32 = 3;
/* FP:context.rs-1114 */ const NUM_PREINTERNED_ANON_BOUND_TYS_V: u32 = 20;
/* FP:context.rs-1115 */ 
/* FP:context.rs-1116 */ // This number may seem high, but it is reached in all but the smallest crates.
/* FP:context.rs-1117 */ const NUM_PREINTERNED_RE_VARS: u32 = 500;
/* FP:context.rs-1118 */ const NUM_PREINTERNED_ANON_RE_BOUNDS_I: u32 = 3;
/* FP:context.rs-1119 */ const NUM_PREINTERNED_ANON_RE_BOUNDS_V: u32 = 20;
/* FP:context.rs-1120 */ 
/* FP:context.rs-1121 */ pub struct CommonTypes<'tcx> {
/* FP:context.rs-1122 */     pub unit: Ty<'tcx>,
/* FP:context.rs-1123 */     pub bool: Ty<'tcx>,
/* FP:context.rs-1124 */     pub char: Ty<'tcx>,
/* FP:context.rs-1125 */     pub isize: Ty<'tcx>,
/* FP:context.rs-1126 */     pub i8: Ty<'tcx>,
/* FP:context.rs-1127 */     pub i16: Ty<'tcx>,
/* FP:context.rs-1128 */     pub i32: Ty<'tcx>,
/* FP:context.rs-1129 */     pub i64: Ty<'tcx>,
/* FP:context.rs-1130 */     pub i128: Ty<'tcx>,
/* FP:context.rs-1131 */     pub usize: Ty<'tcx>,
/* FP:context.rs-1132 */     pub u8: Ty<'tcx>,
/* FP:context.rs-1133 */     pub u16: Ty<'tcx>,
/* FP:context.rs-1134 */     pub u32: Ty<'tcx>,
/* FP:context.rs-1135 */     pub u64: Ty<'tcx>,
/* FP:context.rs-1136 */     pub u128: Ty<'tcx>,
/* FP:context.rs-1137 */     pub f16: Ty<'tcx>,
/* FP:context.rs-1138 */     pub f32: Ty<'tcx>,
/* FP:context.rs-1139 */     pub f64: Ty<'tcx>,
/* FP:context.rs-1140 */     pub f128: Ty<'tcx>,
/* FP:context.rs-1141 */     pub str_: Ty<'tcx>,
/* FP:context.rs-1142 */     pub never: Ty<'tcx>,
/* FP:context.rs-1143 */     pub self_param: Ty<'tcx>,
/* FP:context.rs-1144 */ 
/* FP:context.rs-1145 */     /// Dummy type used for the `Self` of a `TraitRef` created for converting
/* FP:context.rs-1146 */     /// a trait object, and which gets removed in `ExistentialTraitRef`.
/* FP:context.rs-1147 */     /// This type must not appear anywhere in other converted types.
/* FP:context.rs-1148 */     /// `Infer(ty::FreshTy(0))` does the job.
/* FP:context.rs-1149 */     pub trait_object_dummy_self: Ty<'tcx>,
/* FP:context.rs-1150 */ 
/* FP:context.rs-1151 */     /// Pre-interned `Infer(ty::TyVar(n))` for small values of `n`.
/* FP:context.rs-1152 */     pub ty_vars: Vec<Ty<'tcx>>,
/* FP:context.rs-1153 */ 
/* FP:context.rs-1154 */     /// Pre-interned `Infer(ty::FreshTy(n))` for small values of `n`.
/* FP:context.rs-1155 */     pub fresh_tys: Vec<Ty<'tcx>>,
/* FP:context.rs-1156 */ 
/* FP:context.rs-1157 */     /// Pre-interned `Infer(ty::FreshIntTy(n))` for small values of `n`.
/* FP:context.rs-1158 */     pub fresh_int_tys: Vec<Ty<'tcx>>,
/* FP:context.rs-1159 */ 
/* FP:context.rs-1160 */     /// Pre-interned `Infer(ty::FreshFloatTy(n))` for small values of `n`.
/* FP:context.rs-1161 */     pub fresh_float_tys: Vec<Ty<'tcx>>,
/* FP:context.rs-1162 */ 
/* FP:context.rs-1163 */     /// Pre-interned values of the form:
/* FP:context.rs-1164 */     /// `Bound(DebruijnIndex(i), BoundTy { var: v, kind: BoundTyKind::Anon})`
/* FP:context.rs-1165 */     /// for small values of `i` and `v`.
/* FP:context.rs-1166 */     pub anon_bound_tys: Vec<Vec<Ty<'tcx>>>,
/* FP:context.rs-1167 */ }
/* FP:context.rs-1168 */ 
/* FP:context.rs-1169 */ pub struct CommonLifetimes<'tcx> {
/* FP:context.rs-1170 */     /// `ReStatic`
/* FP:context.rs-1171 */     pub re_static: Region<'tcx>,
/* FP:context.rs-1172 */ 
/* FP:context.rs-1173 */     /// Erased region, used outside of type inference.
/* FP:context.rs-1174 */     pub re_erased: Region<'tcx>,
/* FP:context.rs-1175 */ 
/* FP:context.rs-1176 */     /// Pre-interned `ReVar(ty::RegionVar(n))` for small values of `n`.
/* FP:context.rs-1177 */     pub re_vars: Vec<Region<'tcx>>,
/* FP:context.rs-1178 */ 
/* FP:context.rs-1179 */     /// Pre-interned values of the form:
/* FP:context.rs-1180 */     /// `ReBound(DebruijnIndex(i), BoundRegion { var: v, kind: BoundRegionKind::Anon })`
/* FP:context.rs-1181 */     /// for small values of `i` and `v`.
/* FP:context.rs-1182 */     pub anon_re_bounds: Vec<Vec<Region<'tcx>>>,
/* FP:context.rs-1183 */ }
/* FP:context.rs-1184 */ 
/* FP:context.rs-1185 */ pub struct CommonConsts<'tcx> {
/* FP:context.rs-1186 */     pub unit: Const<'tcx>,
/* FP:context.rs-1187 */     pub true_: Const<'tcx>,
/* FP:context.rs-1188 */     pub false_: Const<'tcx>,
/* FP:context.rs-1189 */     /// Use [`ty::ValTree::zst`] instead.
/* FP:context.rs-1190 */     pub(crate) valtree_zst: ValTree<'tcx>,
/* FP:context.rs-1191 */ }
/* FP:context.rs-1192 */ 
/* FP:context.rs-1193 */ impl<'tcx> CommonTypes<'tcx> {
/* FP:context.rs-1194 */     fn new(
/* FP:context.rs-1195 */         interners: &CtxtInterners<'tcx>,
/* FP:context.rs-1196 */         sess: &Session,
/* FP:context.rs-1197 */         untracked: &Untracked,
/* FP:context.rs-1198 */     ) -> CommonTypes<'tcx> {
/* FP:context.rs-1199 */         let mk = |ty| interners.intern_ty(ty, sess, untracked);
/* FP:context.rs-1200 */ 
/* FP:context.rs-1201 */         let ty_vars =
/* FP:context.rs-1202 */             (0..NUM_PREINTERNED_TY_VARS).map(|n| mk(Infer(ty::TyVar(TyVid::from(n))))).collect();
/* FP:context.rs-1203 */         let fresh_tys: Vec<_> =
/* FP:context.rs-1204 */             (0..NUM_PREINTERNED_FRESH_TYS).map(|n| mk(Infer(ty::FreshTy(n)))).collect();
/* FP:context.rs-1205 */         let fresh_int_tys: Vec<_> =
/* FP:context.rs-1206 */             (0..NUM_PREINTERNED_FRESH_INT_TYS).map(|n| mk(Infer(ty::FreshIntTy(n)))).collect();
/* FP:context.rs-1207 */         let fresh_float_tys: Vec<_> =
/* FP:context.rs-1208 */             (0..NUM_PREINTERNED_FRESH_FLOAT_TYS).map(|n| mk(Infer(ty::FreshFloatTy(n)))).collect();
/* FP:context.rs-1209 */ 
/* FP:context.rs-1210 */         let anon_bound_tys = (0..NUM_PREINTERNED_ANON_BOUND_TYS_I)
/* FP:context.rs-1211 */             .map(|i| {
/* FP:context.rs-1212 */                 (0..NUM_PREINTERNED_ANON_BOUND_TYS_V)
/* FP:context.rs-1213 */                     .map(|v| {
/* FP:context.rs-1214 */                         mk(ty::Bound(
/* FP:context.rs-1215 */                             ty::DebruijnIndex::from(i),
/* FP:context.rs-1216 */                             ty::BoundTy { var: ty::BoundVar::from(v), kind: ty::BoundTyKind::Anon },
/* FP:context.rs-1217 */                         ))
/* FP:context.rs-1218 */                     })
/* FP:context.rs-1219 */                     .collect()
/* FP:context.rs-1220 */             })
/* FP:context.rs-1221 */             .collect();
/* FP:context.rs-1222 */ 
/* FP:context.rs-1223 */         CommonTypes {
/* FP:context.rs-1224 */             unit: mk(Tuple(List::empty())),
/* FP:context.rs-1225 */             bool: mk(Bool),
/* FP:context.rs-1226 */             char: mk(Char),
/* FP:context.rs-1227 */             never: mk(Never),
/* FP:context.rs-1228 */             isize: mk(Int(ty::IntTy::Isize)),
/* FP:context.rs-1229 */             i8: mk(Int(ty::IntTy::I8)),
/* FP:context.rs-1230 */             i16: mk(Int(ty::IntTy::I16)),
/* FP:context.rs-1231 */             i32: mk(Int(ty::IntTy::I32)),
/* FP:context.rs-1232 */             i64: mk(Int(ty::IntTy::I64)),
/* FP:context.rs-1233 */             i128: mk(Int(ty::IntTy::I128)),
/* FP:context.rs-1234 */             usize: mk(Uint(ty::UintTy::Usize)),
/* FP:context.rs-1235 */             u8: mk(Uint(ty::UintTy::U8)),
/* FP:context.rs-1236 */             u16: mk(Uint(ty::UintTy::U16)),
/* FP:context.rs-1237 */             u32: mk(Uint(ty::UintTy::U32)),
/* FP:context.rs-1238 */             u64: mk(Uint(ty::UintTy::U64)),
/* FP:context.rs-1239 */             u128: mk(Uint(ty::UintTy::U128)),
/* FP:context.rs-1240 */             f16: mk(Float(ty::FloatTy::F16)),
/* FP:context.rs-1241 */             f32: mk(Float(ty::FloatTy::F32)),
/* FP:context.rs-1242 */             f64: mk(Float(ty::FloatTy::F64)),
/* FP:context.rs-1243 */             f128: mk(Float(ty::FloatTy::F128)),
/* FP:context.rs-1244 */             str_: mk(Str),
/* FP:context.rs-1245 */             self_param: mk(ty::Param(ty::ParamTy { index: 0, name: kw::SelfUpper })),
/* FP:context.rs-1246 */ 
/* FP:context.rs-1247 */             trait_object_dummy_self: fresh_tys[0],
/* FP:context.rs-1248 */ 
/* FP:context.rs-1249 */             ty_vars,
/* FP:context.rs-1250 */             fresh_tys,
/* FP:context.rs-1251 */             fresh_int_tys,
/* FP:context.rs-1252 */             fresh_float_tys,
/* FP:context.rs-1253 */             anon_bound_tys,
/* FP:context.rs-1254 */         }
/* FP:context.rs-1255 */     }
/* FP:context.rs-1256 */ }
/* FP:context.rs-1257 */ 
/* FP:context.rs-1258 */ impl<'tcx> CommonLifetimes<'tcx> {
/* FP:context.rs-1259 */     fn new(interners: &CtxtInterners<'tcx>) -> CommonLifetimes<'tcx> {
/* FP:context.rs-1260 */         let mk = |r| {
/* FP:context.rs-1261 */             Region(Interned::new_unchecked(
/* FP:context.rs-1262 */                 interners.region.intern(r, |r| InternedInSet(interners.arena.alloc(r))).0,
/* FP:context.rs-1263 */             ))
/* FP:context.rs-1264 */         };
/* FP:context.rs-1265 */ 
/* FP:context.rs-1266 */         let re_vars =
/* FP:context.rs-1267 */             (0..NUM_PREINTERNED_RE_VARS).map(|n| mk(ty::ReVar(ty::RegionVid::from(n)))).collect();
/* FP:context.rs-1268 */ 
/* FP:context.rs-1269 */         let anon_re_bounds = (0..NUM_PREINTERNED_ANON_RE_BOUNDS_I)
/* FP:context.rs-1270 */             .map(|i| {
/* FP:context.rs-1271 */                 (0..NUM_PREINTERNED_ANON_RE_BOUNDS_V)
/* FP:context.rs-1272 */                     .map(|v| {
/* FP:context.rs-1273 */                         mk(ty::ReBound(
/* FP:context.rs-1274 */                             ty::DebruijnIndex::from(i),
/* FP:context.rs-1275 */                             ty::BoundRegion {
/* FP:context.rs-1276 */                                 var: ty::BoundVar::from(v),
/* FP:context.rs-1277 */                                 kind: ty::BoundRegionKind::Anon,
/* FP:context.rs-1278 */                             },
/* FP:context.rs-1279 */                         ))
/* FP:context.rs-1280 */                     })
/* FP:context.rs-1281 */                     .collect()
/* FP:context.rs-1282 */             })
/* FP:context.rs-1283 */             .collect();
/* FP:context.rs-1284 */ 
/* FP:context.rs-1285 */         CommonLifetimes {
/* FP:context.rs-1286 */             re_static: mk(ty::ReStatic),
/* FP:context.rs-1287 */             re_erased: mk(ty::ReErased),
/* FP:context.rs-1288 */             re_vars,
/* FP:context.rs-1289 */             anon_re_bounds,
/* FP:context.rs-1290 */         }
/* FP:context.rs-1291 */     }
/* FP:context.rs-1292 */ }
/* FP:context.rs-1293 */ 
/* FP:context.rs-1294 */ impl<'tcx> CommonConsts<'tcx> {
/* FP:context.rs-1295 */     fn new(
/* FP:context.rs-1296 */         interners: &CtxtInterners<'tcx>,
/* FP:context.rs-1297 */         types: &CommonTypes<'tcx>,
/* FP:context.rs-1298 */         sess: &Session,
/* FP:context.rs-1299 */         untracked: &Untracked,
/* FP:context.rs-1300 */     ) -> CommonConsts<'tcx> {
/* FP:context.rs-1301 */         let mk_const = |c| {
/* FP:context.rs-1302 */             interners.intern_const(
/* FP:context.rs-1303 */                 c, sess, // This is only used to create a stable hashing context.
/* FP:context.rs-1304 */                 untracked,
/* FP:context.rs-1305 */             )
/* FP:context.rs-1306 */         };
/* FP:context.rs-1307 */ 
/* FP:context.rs-1308 */         let mk_valtree = |v| {
/* FP:context.rs-1309 */             ty::ValTree(Interned::new_unchecked(
/* FP:context.rs-1310 */                 interners.valtree.intern(v, |v| InternedInSet(interners.arena.alloc(v))).0,
/* FP:context.rs-1311 */             ))
/* FP:context.rs-1312 */         };
/* FP:context.rs-1313 */ 
/* FP:context.rs-1314 */         let valtree_zst = mk_valtree(ty::ValTreeKind::Branch(Box::default()));
/* FP:context.rs-1315 */         let valtree_true = mk_valtree(ty::ValTreeKind::Leaf(ty::ScalarInt::TRUE));
/* FP:context.rs-1316 */         let valtree_false = mk_valtree(ty::ValTreeKind::Leaf(ty::ScalarInt::FALSE));
/* FP:context.rs-1317 */ 
/* FP:context.rs-1318 */         CommonConsts {
/* FP:context.rs-1319 */             unit: mk_const(ty::ConstKind::Value(ty::Value {
/* FP:context.rs-1320 */                 ty: types.unit,
/* FP:context.rs-1321 */                 valtree: valtree_zst,
/* FP:context.rs-1322 */             })),
/* FP:context.rs-1323 */             true_: mk_const(ty::ConstKind::Value(ty::Value {
/* FP:context.rs-1324 */                 ty: types.bool,
/* FP:context.rs-1325 */                 valtree: valtree_true,
/* FP:context.rs-1326 */             })),
/* FP:context.rs-1327 */             false_: mk_const(ty::ConstKind::Value(ty::Value {
/* FP:context.rs-1328 */                 ty: types.bool,
/* FP:context.rs-1329 */                 valtree: valtree_false,
/* FP:context.rs-1330 */             })),
/* FP:context.rs-1331 */             valtree_zst,
/* FP:context.rs-1332 */         }
/* FP:context.rs-1333 */     }
/* FP:context.rs-1334 */ }
/* FP:context.rs-1335 */ 
/* FP:context.rs-1336 */ /// This struct contains information regarding a free parameter region,
/* FP:context.rs-1337 */ /// either a `ReEarlyParam` or `ReLateParam`.
/* FP:context.rs-1338 */ #[derive(Debug)]
/* FP:context.rs-1339 */ pub struct FreeRegionInfo {
/* FP:context.rs-1340 */     /// `LocalDefId` of the scope.
/* FP:context.rs-1341 */     pub scope: LocalDefId,
/* FP:context.rs-1342 */     /// the `DefId` of the free region.
/* FP:context.rs-1343 */     pub region_def_id: DefId,
/* FP:context.rs-1344 */     /// checks if bound region is in Impl Item
/* FP:context.rs-1345 */     pub is_impl_item: bool,
/* FP:context.rs-1346 */ }
/* FP:context.rs-1347 */ 
/* FP:context.rs-1348 */ /// This struct should only be created by `create_def`.
/* FP:context.rs-1349 */ #[derive(Copy, Clone)]
/* FP:context.rs-1350 */ pub struct TyCtxtFeed<'tcx, KEY: Copy> {
/* FP:context.rs-1351 */     pub tcx: TyCtxt<'tcx>,
/* FP:context.rs-1352 */     // Do not allow direct access, as downstream code must not mutate this field.
/* FP:context.rs-1353 */     key: KEY,
/* FP:context.rs-1354 */ }
/* FP:context.rs-1355 */ 
/* FP:context.rs-1356 */ /// Never return a `Feed` from a query. Only queries that create a `DefId` are
/* FP:context.rs-1357 */ /// allowed to feed queries for that `DefId`.
/* FP:context.rs-1358 */ impl<KEY: Copy, CTX> !HashStable<CTX> for TyCtxtFeed<'_, KEY> {}
/* FP:context.rs-1359 */ 
/* FP:context.rs-1360 */ /// The same as `TyCtxtFeed`, but does not contain a `TyCtxt`.
/* FP:context.rs-1361 */ /// Use this to pass around when you have a `TyCtxt` elsewhere.
/* FP:context.rs-1362 */ /// Just an optimization to save space and not store hundreds of
/* FP:context.rs-1363 */ /// `TyCtxtFeed` in the resolver.
/* FP:context.rs-1364 */ #[derive(Copy, Clone)]
/* FP:context.rs-1365 */ pub struct Feed<'tcx, KEY: Copy> {
/* FP:context.rs-1366 */     _tcx: PhantomData<TyCtxt<'tcx>>,
/* FP:context.rs-1367 */     // Do not allow direct access, as downstream code must not mutate this field.
/* FP:context.rs-1368 */     key: KEY,
/* FP:context.rs-1369 */ }
/* FP:context.rs-1370 */ 
/* FP:context.rs-1371 */ /// Never return a `Feed` from a query. Only queries that create a `DefId` are
/* FP:context.rs-1372 */ /// allowed to feed queries for that `DefId`.
/* FP:context.rs-1373 */ impl<KEY: Copy, CTX> !HashStable<CTX> for Feed<'_, KEY> {}
/* FP:context.rs-1374 */ 
/* FP:context.rs-1375 */ impl<T: fmt::Debug + Copy> fmt::Debug for Feed<'_, T> {
/* FP:context.rs-1376 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:context.rs-1377 */         self.key.fmt(f)
/* FP:context.rs-1378 */     }
/* FP:context.rs-1379 */ }
/* FP:context.rs-1380 */ 
/* FP:context.rs-1381 */ /// Some workarounds to use cases that cannot use `create_def`.
/* FP:context.rs-1382 */ /// Do not add new ways to create `TyCtxtFeed` without consulting
/* FP:context.rs-1383 */ /// with T-compiler and making an analysis about why your addition
/* FP:context.rs-1384 */ /// does not cause incremental compilation issues.
/* FP:context.rs-1385 */ impl<'tcx> TyCtxt<'tcx> {
/* FP:context.rs-1386 */     /// Can only be fed before queries are run, and is thus exempt from any
/* FP:context.rs-1387 */     /// incremental issues. Do not use except for the initial query feeding.
/* FP:context.rs-1388 */     pub fn feed_unit_query(self) -> TyCtxtFeed<'tcx, ()> {
/* FP:context.rs-1389 */         self.dep_graph.assert_ignored();
/* FP:context.rs-1390 */         TyCtxtFeed { tcx: self, key: () }
/* FP:context.rs-1391 */     }
/* FP:context.rs-1392 */ 
/* FP:context.rs-1393 */     /// Only used in the resolver to register the `CRATE_DEF_ID` `DefId` and feed
/* FP:context.rs-1394 */     /// some queries for it. It will panic if used twice.
/* FP:context.rs-1395 */     pub fn create_local_crate_def_id(self, span: Span) -> TyCtxtFeed<'tcx, LocalDefId> {
/* FP:context.rs-1396 */         let key = self.untracked().source_span.push(span);
/* FP:context.rs-1397 */         assert_eq!(key, CRATE_DEF_ID);
/* FP:context.rs-1398 */         TyCtxtFeed { tcx: self, key }
/* FP:context.rs-1399 */     }
/* FP:context.rs-1400 */ 
/* FP:context.rs-1401 */     /// In order to break cycles involving `AnonConst`, we need to set the expected type by side
/* FP:context.rs-1402 */     /// effect. However, we do not want this as a general capability, so this interface restricts
/* FP:context.rs-1403 */     /// to the only allowed case.
/* FP:context.rs-1404 */     pub fn feed_anon_const_type(self, key: LocalDefId, value: ty::EarlyBinder<'tcx, Ty<'tcx>>) {
/* FP:context.rs-1405 */         debug_assert_eq!(self.def_kind(key), DefKind::AnonConst);
/* FP:context.rs-1406 */         TyCtxtFeed { tcx: self, key }.type_of(value)
/* FP:context.rs-1407 */     }
/* FP:context.rs-1408 */ }
/* FP:context.rs-1409 */ 
/* FP:context.rs-1410 */ impl<'tcx, KEY: Copy> TyCtxtFeed<'tcx, KEY> {
/* FP:context.rs-1411 */     #[inline(always)]
/* FP:context.rs-1412 */     pub fn key(&self) -> KEY {
/* FP:context.rs-1413 */         self.key
/* FP:context.rs-1414 */     }
/* FP:context.rs-1415 */ 
/* FP:context.rs-1416 */     #[inline(always)]
/* FP:context.rs-1417 */     pub fn downgrade(self) -> Feed<'tcx, KEY> {
/* FP:context.rs-1418 */         Feed { _tcx: PhantomData, key: self.key }
/* FP:context.rs-1419 */     }
/* FP:context.rs-1420 */ }
/* FP:context.rs-1421 */ 
/* FP:context.rs-1422 */ impl<'tcx, KEY: Copy> Feed<'tcx, KEY> {
/* FP:context.rs-1423 */     #[inline(always)]
/* FP:context.rs-1424 */     pub fn key(&self) -> KEY {
/* FP:context.rs-1425 */         self.key
/* FP:context.rs-1426 */     }
/* FP:context.rs-1427 */ 
/* FP:context.rs-1428 */     #[inline(always)]
/* FP:context.rs-1429 */     pub fn upgrade(self, tcx: TyCtxt<'tcx>) -> TyCtxtFeed<'tcx, KEY> {
/* FP:context.rs-1430 */         TyCtxtFeed { tcx, key: self.key }
/* FP:context.rs-1431 */     }
/* FP:context.rs-1432 */ }
/* FP:context.rs-1433 */ 
/* FP:context.rs-1434 */ impl<'tcx> TyCtxtFeed<'tcx, LocalDefId> {
/* FP:context.rs-1435 */     #[inline(always)]
/* FP:context.rs-1436 */     pub fn def_id(&self) -> LocalDefId {
/* FP:context.rs-1437 */         self.key
/* FP:context.rs-1438 */     }
/* FP:context.rs-1439 */ 
/* FP:context.rs-1440 */     // Caller must ensure that `self.key` ID is indeed an owner.
/* FP:context.rs-1441 */     pub fn feed_owner_id(&self) -> TyCtxtFeed<'tcx, hir::OwnerId> {
/* FP:context.rs-1442 */         TyCtxtFeed { tcx: self.tcx, key: hir::OwnerId { def_id: self.key } }
/* FP:context.rs-1443 */     }
/* FP:context.rs-1444 */ 
/* FP:context.rs-1445 */     // Fills in all the important parts needed by HIR queries
/* FP:context.rs-1446 */     pub fn feed_hir(&self) {
/* FP:context.rs-1447 */         self.local_def_id_to_hir_id(HirId::make_owner(self.def_id()));
/* FP:context.rs-1448 */ 
/* FP:context.rs-1449 */         let node = hir::OwnerNode::Synthetic;
/* FP:context.rs-1450 */         let bodies = Default::default();
/* FP:context.rs-1451 */         let attrs = hir::AttributeMap::EMPTY;
/* FP:context.rs-1452 */ 
/* FP:context.rs-1453 */         let crate::rustc_middle::hir::Hashes { opt_hash_including_bodies, .. } =
/* FP:context.rs-1454 */             self.tcx.hash_owner_nodes(node, &bodies, &attrs.map, &[], attrs.define_opaque);
/* FP:context.rs-1455 */         let node = node.into();
/* FP:context.rs-1456 */         self.opt_hir_owner_nodes(Some(self.tcx.arena.alloc(hir::OwnerNodes {
/* FP:context.rs-1457 */             opt_hash_including_bodies,
/* FP:context.rs-1458 */             nodes: IndexVec::from_elem_n(
/* FP:context.rs-1459 */                 hir::ParentedNode { parent: hir::ItemLocalId::INVALID, node },
/* FP:context.rs-1460 */                 1,
/* FP:context.rs-1461 */             ),
/* FP:context.rs-1462 */             bodies,
/* FP:context.rs-1463 */         })));
/* FP:context.rs-1464 */         self.feed_owner_id().hir_attr_map(attrs);
/* FP:context.rs-1465 */     }
/* FP:context.rs-1466 */ }
/* FP:context.rs-1467 */ 
/* FP:context.rs-1468 */ /// The central data structure of the compiler. It stores references
/* FP:context.rs-1469 */ /// to the various **arenas** and also houses the results of the
/* FP:context.rs-1470 */ /// various **compiler queries** that have been performed. See the
/* FP:context.rs-1471 */ /// [rustc dev guide] for more details.
/* FP:context.rs-1472 */ ///
/* FP:context.rs-1473 */ /// [rustc dev guide]: https://rustc-dev-guide.rust-lang.org/ty.html
/* FP:context.rs-1474 */ ///
/* FP:context.rs-1475 */ /// An implementation detail: `TyCtxt` is a wrapper type for [GlobalCtxt],
/* FP:context.rs-1476 */ /// which is the struct that actually holds all the data. `TyCtxt` derefs to
/* FP:context.rs-1477 */ /// `GlobalCtxt`, and in practice `TyCtxt` is passed around everywhere, and all
/* FP:context.rs-1478 */ /// operations are done via `TyCtxt`. A `TyCtxt` is obtained for a `GlobalCtxt`
/* FP:context.rs-1479 */ /// by calling `enter` with a closure `f`. That function creates both the
/* FP:context.rs-1480 */ /// `TyCtxt`, and an `ImplicitCtxt` around it that is put into TLS. Within `f`:
/* FP:context.rs-1481 */ /// - The `ImplicitCtxt` is available implicitly via TLS.
/* FP:context.rs-1482 */ /// - The `TyCtxt` is available explicitly via the `tcx` parameter, and also
/* FP:context.rs-1483 */ ///   implicitly within the `ImplicitCtxt`. Explicit access is preferred when
/* FP:context.rs-1484 */ ///   possible.
/* FP:context.rs-1485 */ #[derive(Copy, Clone)]
/* FP:context.rs-1486 */ #[rustc_diagnostic_item = "TyCtxt"]
/* FP:context.rs-1487 */ #[rustc_pass_by_value]
/* FP:context.rs-1488 */ pub struct TyCtxt<'tcx> {
/* FP:context.rs-1489 */     gcx: &'tcx GlobalCtxt<'tcx>,
/* FP:context.rs-1490 */ }
/* FP:context.rs-1491 */ 
/* FP:context.rs-1492 */ impl<'tcx> LintEmitter for TyCtxt<'tcx> {
/* FP:context.rs-1493 */     type Id = HirId;
/* FP:context.rs-1494 */ 
/* FP:context.rs-1495 */     fn emit_node_span_lint(
/* FP:context.rs-1496 */         self,
/* FP:context.rs-1497 */         lint: &'static Lint,
/* FP:context.rs-1498 */         hir_id: HirId,
/* FP:context.rs-1499 */         span: impl Into<MultiSpan>,
/* FP:context.rs-1500 */         decorator: impl for<'a> LintDiagnostic<'a, ()>,
/* FP:context.rs-1501 */     ) {
/* FP:context.rs-1502 */         self.emit_node_span_lint(lint, hir_id, span, decorator);
/* FP:context.rs-1503 */     }
/* FP:context.rs-1504 */ }
/* FP:context.rs-1505 */ 
/* FP:context.rs-1506 */ // Explicitly implement `DynSync` and `DynSend` for `TyCtxt` to short circuit trait resolution. Its
/* FP:context.rs-1507 */ // field are asserted to implement these traits below, so this is trivially safe, and it greatly
/* FP:context.rs-1508 */ // speeds-up compilation of this crate and its dependents.
/* FP:context.rs-1509 */ unsafe impl DynSend for TyCtxt<'_> {}
/* FP:context.rs-1510 */ unsafe impl DynSync for TyCtxt<'_> {}
/* FP:context.rs-1511 */ fn _assert_tcx_fields() {
/* FP:context.rs-1512 */     sync::assert_dyn_sync::<&'_ GlobalCtxt<'_>>();
/* FP:context.rs-1513 */     sync::assert_dyn_send::<&'_ GlobalCtxt<'_>>();
/* FP:context.rs-1514 */ }
/* FP:context.rs-1515 */ 
/* FP:context.rs-1516 */ impl<'tcx> Deref for TyCtxt<'tcx> {
/* FP:context.rs-1517 */     type Target = &'tcx GlobalCtxt<'tcx>;
/* FP:context.rs-1518 */     #[inline(always)]
/* FP:context.rs-1519 */     fn deref(&self) -> &Self::Target {
/* FP:context.rs-1520 */         &self.gcx
/* FP:context.rs-1521 */     }
/* FP:context.rs-1522 */ }
/* FP:context.rs-1523 */ 
/* FP:context.rs-1524 */ /// See [TyCtxt] for details about this type.
/* FP:context.rs-1525 */ pub struct GlobalCtxt<'tcx> {
/* FP:context.rs-1526 */     pub arena: &'tcx WorkerLocal<Arena<'tcx>>,
/* FP:context.rs-1527 */     pub hir_arena: &'tcx WorkerLocal<hir::Arena<'tcx>>,
/* FP:context.rs-1528 */ 
/* FP:context.rs-1529 */     interners: CtxtInterners<'tcx>,
/* FP:context.rs-1530 */ 
/* FP:context.rs-1531 */     pub sess: &'tcx Session,
/* FP:context.rs-1532 */     crate_types: Vec<CrateType>,
/* FP:context.rs-1533 */     /// The `stable_crate_id` is constructed out of the crate name and all the
/* FP:context.rs-1534 */     /// `-C metadata` arguments passed to the compiler. Its value forms a unique
/* FP:context.rs-1535 */     /// global identifier for the crate. It is used to allow multiple crates
/* FP:context.rs-1536 */     /// with the same name to coexist. See the
/* FP:context.rs-1537 */     /// `rustc_symbol_mangling` crate for more information.
/* FP:context.rs-1538 */     stable_crate_id: StableCrateId,
/* FP:context.rs-1539 */ 
/* FP:context.rs-1540 */     pub dep_graph: DepGraph,
/* FP:context.rs-1541 */ 
/* FP:context.rs-1542 */     pub prof: SelfProfilerRef,
/* FP:context.rs-1543 */ 
/* FP:context.rs-1544 */     /// Common types, pre-interned for your convenience.
/* FP:context.rs-1545 */     pub types: CommonTypes<'tcx>,
/* FP:context.rs-1546 */ 
/* FP:context.rs-1547 */     /// Common lifetimes, pre-interned for your convenience.
/* FP:context.rs-1548 */     pub lifetimes: CommonLifetimes<'tcx>,
/* FP:context.rs-1549 */ 
/* FP:context.rs-1550 */     /// Common consts, pre-interned for your convenience.
/* FP:context.rs-1551 */     pub consts: CommonConsts<'tcx>,
/* FP:context.rs-1552 */ 
/* FP:context.rs-1553 */     /// Hooks to be able to register functions in other crates that can then still
/* FP:context.rs-1554 */     /// be called from rustc_middle.
/* FP:context.rs-1555 */     pub(crate) hooks: crate::hooks::Providers,
/* FP:context.rs-1556 */ 
/* FP:context.rs-1557 */     untracked: Untracked,
/* FP:context.rs-1558 */ 
/* FP:context.rs-1559 */     pub query_system: QuerySystem<'tcx>,
/* FP:context.rs-1560 */     pub(crate) query_kinds: &'tcx [DepKindStruct<'tcx>],
/* FP:context.rs-1561 */ 
/* FP:context.rs-1562 */     // Internal caches for metadata decoding. No need to track deps on this.
/* FP:context.rs-1563 */     pub ty_rcache: Lock<FxHashMap<ty::CReaderCacheKey, Ty<'tcx>>>,
/* FP:context.rs-1564 */ 
/* FP:context.rs-1565 */     /// Caches the results of trait selection. This cache is used
/* FP:context.rs-1566 */     /// for things that do not have to do with the parameters in scope.
/* FP:context.rs-1567 */     pub selection_cache: traits::SelectionCache<'tcx, ty::TypingEnv<'tcx>>,
/* FP:context.rs-1568 */ 
/* FP:context.rs-1569 */     /// Caches the results of trait evaluation. This cache is used
/* FP:context.rs-1570 */     /// for things that do not have to do with the parameters in scope.
/* FP:context.rs-1571 */     /// Merge this with `selection_cache`?
/* FP:context.rs-1572 */     pub evaluation_cache: traits::EvaluationCache<'tcx, ty::TypingEnv<'tcx>>,
/* FP:context.rs-1573 */ 
/* FP:context.rs-1574 */     /// Caches the results of goal evaluation in the new solver.
/* FP:context.rs-1575 */     pub new_solver_evaluation_cache: Lock<search_graph::GlobalCache<TyCtxt<'tcx>>>,
/* FP:context.rs-1576 */     pub new_solver_canonical_param_env_cache:
/* FP:context.rs-1577 */         Lock<FxHashMap<ty::ParamEnv<'tcx>, ty::CanonicalParamEnvCacheEntry<TyCtxt<'tcx>>>>,
/* FP:context.rs-1578 */ 
/* FP:context.rs-1579 */     pub canonical_param_env_cache: CanonicalParamEnvCache<'tcx>,
/* FP:context.rs-1580 */ 
/* FP:context.rs-1581 */     /// Caches the index of the highest bound var in clauses in a canonical binder.
/* FP:context.rs-1582 */     pub highest_var_in_clauses_cache: Lock<FxHashMap<ty::Clauses<'tcx>, usize>>,
/* FP:context.rs-1583 */     /// Caches the instantiation of a canonical binder given a set of args.
/* FP:context.rs-1584 */     pub clauses_cache:
/* FP:context.rs-1585 */         Lock<FxHashMap<(ty::Clauses<'tcx>, &'tcx [ty::GenericArg<'tcx>]), ty::Clauses<'tcx>>>,
/* FP:context.rs-1586 */ 
/* FP:context.rs-1587 */     /// Data layout specification for the current target.
/* FP:context.rs-1588 */     pub data_layout: TargetDataLayout,
/* FP:context.rs-1589 */ 
/* FP:context.rs-1590 */     /// Stores memory for globals (statics/consts).
/* FP:context.rs-1591 */     pub(crate) alloc_map: interpret::AllocMap<'tcx>,
/* FP:context.rs-1592 */ 
/* FP:context.rs-1593 */     current_gcx: CurrentGcx,
/* FP:context.rs-1594 */ 
/* FP:context.rs-1595 */     /// A jobserver reference used to release then acquire a token while waiting on a query.
/* FP:context.rs-1596 */     pub jobserver_proxy: Arc<Proxy>,
/* FP:context.rs-1597 */ }
/* FP:context.rs-1598 */ 
/* FP:context.rs-1599 */ impl<'tcx> GlobalCtxt<'tcx> {
/* FP:context.rs-1600 */     /// Installs `self` in a `TyCtxt` and `ImplicitCtxt` for the duration of
/* FP:context.rs-1601 */     /// `f`.
/* FP:context.rs-1602 */     pub fn enter<F, R>(&'tcx self, f: F) -> R
/* FP:context.rs-1603 */     where
/* FP:context.rs-1604 */         F: FnOnce(TyCtxt<'tcx>) -> R,
/* FP:context.rs-1605 */     {
/* FP:context.rs-1606 */         let icx = tls::ImplicitCtxt::new(self);
/* FP:context.rs-1607 */ 
/* FP:context.rs-1608 */         // Reset `current_gcx` to `None` when we exit.
/* FP:context.rs-1609 */         let _on_drop = defer(move || {
/* FP:context.rs-1610 */             *self.current_gcx.value.write() = None;
/* FP:context.rs-1611 */         });
/* FP:context.rs-1612 */ 
/* FP:context.rs-1613 */         // Set this `GlobalCtxt` as the current one.
/* FP:context.rs-1614 */         {
/* FP:context.rs-1615 */             let mut guard = self.current_gcx.value.write();
/* FP:context.rs-1616 */             assert!(guard.is_none(), "no `GlobalCtxt` is currently set");
/* FP:context.rs-1617 */             *guard = Some(self as *const _ as *const ());
/* FP:context.rs-1618 */         }
/* FP:context.rs-1619 */ 
/* FP:context.rs-1620 */         tls::enter_context(&icx, || f(icx.tcx))
/* FP:context.rs-1621 */     }
/* FP:context.rs-1622 */ }
/* FP:context.rs-1623 */ 
/* FP:context.rs-1624 */ /// This is used to get a reference to a `GlobalCtxt` if one is available.
/* FP:context.rs-1625 */ ///
/* FP:context.rs-1626 */ /// This is needed to allow the deadlock handler access to `GlobalCtxt` to look for query cycles.
/* FP:context.rs-1627 */ /// It cannot use the `TLV` global because that's only guaranteed to be defined on the thread
/* FP:context.rs-1628 */ /// creating the `GlobalCtxt`. Other threads have access to the `TLV` only inside Rayon jobs, but
/* FP:context.rs-1629 */ /// the deadlock handler is not called inside such a job.
/* FP:context.rs-1630 */ #[derive(Clone)]
/* FP:context.rs-1631 */ pub struct CurrentGcx {
/* FP:context.rs-1632 */     /// This stores a pointer to a `GlobalCtxt`. This is set to `Some` inside `GlobalCtxt::enter`
/* FP:context.rs-1633 */     /// and reset to `None` when that function returns or unwinds.
/* FP:context.rs-1634 */     value: Arc<RwLock<Option<*const ()>>>,
/* FP:context.rs-1635 */ }
/* FP:context.rs-1636 */ 
/* FP:context.rs-1637 */ unsafe impl DynSend for CurrentGcx {}
/* FP:context.rs-1638 */ unsafe impl DynSync for CurrentGcx {}
/* FP:context.rs-1639 */ 
/* FP:context.rs-1640 */ impl CurrentGcx {
/* FP:context.rs-1641 */     pub fn new() -> Self {
/* FP:context.rs-1642 */         Self { value: Arc::new(RwLock::new(None)) }
/* FP:context.rs-1643 */     }
/* FP:context.rs-1644 */ 
/* FP:context.rs-1645 */     pub fn access<R>(&self, f: impl for<'tcx> FnOnce(&'tcx GlobalCtxt<'tcx>) -> R) -> R {
/* FP:context.rs-1646 */         let read_guard = self.value.read();
/* FP:context.rs-1647 */         let gcx: *const GlobalCtxt<'_> = read_guard.unwrap() as *const _;
/* FP:context.rs-1648 */         // SAFETY: We hold the read lock for the `GlobalCtxt` pointer. That prevents
/* FP:context.rs-1649 */         // `GlobalCtxt::enter` from returning as it would first acquire the write lock.
/* FP:context.rs-1650 */         // This ensures the `GlobalCtxt` is live during `f`.
/* FP:context.rs-1651 */         f(unsafe { &*gcx })
/* FP:context.rs-1652 */     }
/* FP:context.rs-1653 */ }
/* FP:context.rs-1654 */ 
/* FP:context.rs-1655 */ impl<'tcx> TyCtxt<'tcx> {
/* FP:context.rs-1656 */     pub fn has_typeck_results(self, def_id: LocalDefId) -> bool {
/* FP:context.rs-1657 */         // Closures' typeck results come from their outermost function,
/* FP:context.rs-1658 */         // as they are part of the same "inference environment".
/* FP:context.rs-1659 */         let typeck_root_def_id = self.typeck_root_def_id(def_id.to_def_id());
/* FP:context.rs-1660 */         if typeck_root_def_id != def_id.to_def_id() {
/* FP:context.rs-1661 */             return self.has_typeck_results(typeck_root_def_id.expect_local());
/* FP:context.rs-1662 */         }
/* FP:context.rs-1663 */ 
/* FP:context.rs-1664 */         self.hir_node_by_def_id(def_id).body_id().is_some()
/* FP:context.rs-1665 */     }
/* FP:context.rs-1666 */ 
/* FP:context.rs-1667 */     /// Expects a body and returns its codegen attributes.
/* FP:context.rs-1668 */     ///
/* FP:context.rs-1669 */     /// Unlike `codegen_fn_attrs`, this returns `CodegenFnAttrs::EMPTY` for
/* FP:context.rs-1670 */     /// constants.
/* FP:context.rs-1671 */     pub fn body_codegen_attrs(self, def_id: DefId) -> &'tcx CodegenFnAttrs {
/* FP:context.rs-1672 */         let def_kind = self.def_kind(def_id);
/* FP:context.rs-1673 */         if def_kind.has_codegen_attrs() {
/* FP:context.rs-1674 */             self.codegen_fn_attrs(def_id)
/* FP:context.rs-1675 */         } else if matches!(
/* FP:context.rs-1676 */             def_kind,
/* FP:context.rs-1677 */             DefKind::AnonConst
/* FP:context.rs-1678 */                 | DefKind::AssocConst
/* FP:context.rs-1679 */                 | DefKind::Const
/* FP:context.rs-1680 */                 | DefKind::InlineConst
/* FP:context.rs-1681 */                 | DefKind::GlobalAsm
/* FP:context.rs-1682 */         ) {
/* FP:context.rs-1683 */             CodegenFnAttrs::EMPTY
/* FP:context.rs-1684 */         } else {
/* FP:context.rs-1685 */             bug!(
/* FP:context.rs-1686 */                 "body_codegen_fn_attrs called on unexpected definition: {:?} {:?}",
/* FP:context.rs-1687 */                 def_id,
/* FP:context.rs-1688 */                 def_kind
/* FP:context.rs-1689 */             )
/* FP:context.rs-1690 */         }
/* FP:context.rs-1691 */     }
/* FP:context.rs-1692 */ 
/* FP:context.rs-1693 */     pub fn alloc_steal_thir(self, thir: Thir<'tcx>) -> &'tcx Steal<Thir<'tcx>> {
/* FP:context.rs-1694 */         self.arena.alloc(Steal::new(thir))
/* FP:context.rs-1695 */     }
/* FP:context.rs-1696 */ 
/* FP:context.rs-1697 */     pub fn alloc_steal_mir(self, mir: Body<'tcx>) -> &'tcx Steal<Body<'tcx>> {
/* FP:context.rs-1698 */         self.arena.alloc(Steal::new(mir))
/* FP:context.rs-1699 */     }
/* FP:context.rs-1700 */ 
/* FP:context.rs-1701 */     pub fn alloc_steal_promoted(
/* FP:context.rs-1702 */         self,
/* FP:context.rs-1703 */         promoted: IndexVec<Promoted, Body<'tcx>>,
/* FP:context.rs-1704 */     ) -> &'tcx Steal<IndexVec<Promoted, Body<'tcx>>> {
/* FP:context.rs-1705 */         self.arena.alloc(Steal::new(promoted))
/* FP:context.rs-1706 */     }
/* FP:context.rs-1707 */ 
/* FP:context.rs-1708 */     pub fn mk_adt_def(
/* FP:context.rs-1709 */         self,
/* FP:context.rs-1710 */         did: DefId,
/* FP:context.rs-1711 */         kind: AdtKind,
/* FP:context.rs-1712 */         variants: IndexVec<VariantIdx, ty::VariantDef>,
/* FP:context.rs-1713 */         repr: ReprOptions,
/* FP:context.rs-1714 */     ) -> ty::AdtDef<'tcx> {
/* FP:context.rs-1715 */         self.mk_adt_def_from_data(ty::AdtDefData::new(self, did, kind, variants, repr))
/* FP:context.rs-1716 */     }
/* FP:context.rs-1717 */ 
/* FP:context.rs-1718 */     /// Allocates a read-only byte or string literal for `mir::interpret` with alignment 1.
/* FP:context.rs-1719 */     /// Returns the same `AllocId` if called again with the same bytes.
/* FP:context.rs-1720 */     pub fn allocate_bytes_dedup<'a>(
/* FP:context.rs-1721 */         self,
/* FP:context.rs-1722 */         bytes: impl Into<Cow<'a, [u8]>>,
/* FP:context.rs-1723 */         salt: usize,
/* FP:context.rs-1724 */     ) -> interpret::AllocId {
/* FP:context.rs-1725 */         // Create an allocation that just contains these bytes.
/* FP:context.rs-1726 */         let alloc = interpret::Allocation::from_bytes_byte_aligned_immutable(bytes, ());
/* FP:context.rs-1727 */         let alloc = self.mk_const_alloc(alloc);
/* FP:context.rs-1728 */         self.reserve_and_set_memory_dedup(alloc, salt)
/* FP:context.rs-1729 */     }
/* FP:context.rs-1730 */ 
/* FP:context.rs-1731 */     /// Traits added on all bounds by default, excluding `Sized` which is treated separately.
/* FP:context.rs-1732 */     pub fn default_traits(self) -> &'static [crate::rustc_hir::LangItem] {
/* FP:context.rs-1733 */         if self.sess.opts.unstable_opts.experimental_default_bounds {
/* FP:context.rs-1734 */             &[
/* FP:context.rs-1735 */                 LangItem::DefaultTrait1,
/* FP:context.rs-1736 */                 LangItem::DefaultTrait2,
/* FP:context.rs-1737 */                 LangItem::DefaultTrait3,
/* FP:context.rs-1738 */                 LangItem::DefaultTrait4,
/* FP:context.rs-1739 */             ]
/* FP:context.rs-1740 */         } else {
/* FP:context.rs-1741 */             &[]
/* FP:context.rs-1742 */         }
/* FP:context.rs-1743 */     }
/* FP:context.rs-1744 */ 
/* FP:context.rs-1745 */     pub fn is_default_trait(self, def_id: DefId) -> bool {
/* FP:context.rs-1746 */         self.default_traits()
/* FP:context.rs-1747 */             .iter()
/* FP:context.rs-1748 */             .any(|&default_trait| self.lang_items().get(default_trait) == Some(def_id))
/* FP:context.rs-1749 */     }
/* FP:context.rs-1750 */ 
/* FP:context.rs-1751 */     /// Returns a range of the start/end indices specified with the
/* FP:context.rs-1752 */     /// `rustc_layout_scalar_valid_range` attribute.
/* FP:context.rs-1753 */     // FIXME(eddyb) this is an awkward spot for this method, maybe move it?
/* FP:context.rs-1754 */     pub fn layout_scalar_valid_range(self, def_id: DefId) -> (Bound<u128>, Bound<u128>) {
/* FP:context.rs-1755 */         let start = find_attr!(self.get_all_attrs(def_id), AttributeKind::RustcLayoutScalarValidRangeStart(n, _) => Bound::Included(**n)).unwrap_or(Bound::Unbounded);
/* FP:context.rs-1756 */         let end = find_attr!(self.get_all_attrs(def_id), AttributeKind::RustcLayoutScalarValidRangeEnd(n, _) => Bound::Included(**n)).unwrap_or(Bound::Unbounded);
/* FP:context.rs-1757 */         (start, end)
/* FP:context.rs-1758 */     }
/* FP:context.rs-1759 */ 
/* FP:context.rs-1760 */     pub fn lift<T: Lift<TyCtxt<'tcx>>>(self, value: T) -> Option<T::Lifted> {
/* FP:context.rs-1761 */         value.lift_to_interner(self)
/* FP:context.rs-1762 */     }
/* FP:context.rs-1763 */ 
/* FP:context.rs-1764 */     /// Creates a type context. To use the context call `fn enter` which
/* FP:context.rs-1765 */     /// provides a `TyCtxt`.
/* FP:context.rs-1766 */     ///
/* FP:context.rs-1767 */     /// By only providing the `TyCtxt` inside of the closure we enforce that the type
/* FP:context.rs-1768 */     /// context and any interned value (types, args, etc.) can only be used while `ty::tls`
/* FP:context.rs-1769 */     /// has a valid reference to the context, to allow formatting values that need it.
/* FP:context.rs-1770 */     pub fn create_global_ctxt<T>(
/* FP:context.rs-1771 */         gcx_cell: &'tcx OnceLock<GlobalCtxt<'tcx>>,
/* FP:context.rs-1772 */         s: &'tcx Session,
/* FP:context.rs-1773 */         crate_types: Vec<CrateType>,
/* FP:context.rs-1774 */         stable_crate_id: StableCrateId,
/* FP:context.rs-1775 */         arena: &'tcx WorkerLocal<Arena<'tcx>>,
/* FP:context.rs-1776 */         hir_arena: &'tcx WorkerLocal<hir::Arena<'tcx>>,
/* FP:context.rs-1777 */         untracked: Untracked,
/* FP:context.rs-1778 */         dep_graph: DepGraph,
/* FP:context.rs-1779 */         query_kinds: &'tcx [DepKindStruct<'tcx>],
/* FP:context.rs-1780 */         query_system: QuerySystem<'tcx>,
/* FP:context.rs-1781 */         hooks: crate::hooks::Providers,
/* FP:context.rs-1782 */         current_gcx: CurrentGcx,
/* FP:context.rs-1783 */         jobserver_proxy: Arc<Proxy>,
/* FP:context.rs-1784 */         f: impl FnOnce(TyCtxt<'tcx>) -> T,
/* FP:context.rs-1785 */     ) -> T {
/* FP:context.rs-1786 */         let data_layout = s.target.parse_data_layout().unwrap_or_else(|err| {
/* FP:context.rs-1787 */             s.dcx().emit_fatal(err);
/* FP:context.rs-1788 */         });
/* FP:context.rs-1789 */         let interners = CtxtInterners::new(arena);
/* FP:context.rs-1790 */         let common_types = CommonTypes::new(&interners, s, &untracked);
/* FP:context.rs-1791 */         let common_lifetimes = CommonLifetimes::new(&interners);
/* FP:context.rs-1792 */         let common_consts = CommonConsts::new(&interners, &common_types, s, &untracked);
/* FP:context.rs-1793 */ 
/* FP:context.rs-1794 */         let gcx = gcx_cell.get_or_init(|| GlobalCtxt {
/* FP:context.rs-1795 */             sess: s,
/* FP:context.rs-1796 */             crate_types,
/* FP:context.rs-1797 */             stable_crate_id,
/* FP:context.rs-1798 */             arena,
/* FP:context.rs-1799 */             hir_arena,
/* FP:context.rs-1800 */             interners,
/* FP:context.rs-1801 */             dep_graph,
/* FP:context.rs-1802 */             hooks,
/* FP:context.rs-1803 */             prof: s.prof.clone(),
/* FP:context.rs-1804 */             types: common_types,
/* FP:context.rs-1805 */             lifetimes: common_lifetimes,
/* FP:context.rs-1806 */             consts: common_consts,
/* FP:context.rs-1807 */             untracked,
/* FP:context.rs-1808 */             query_system,
/* FP:context.rs-1809 */             query_kinds,
/* FP:context.rs-1810 */             ty_rcache: Default::default(),
/* FP:context.rs-1811 */             selection_cache: Default::default(),
/* FP:context.rs-1812 */             evaluation_cache: Default::default(),
/* FP:context.rs-1813 */             new_solver_evaluation_cache: Default::default(),
/* FP:context.rs-1814 */             new_solver_canonical_param_env_cache: Default::default(),
/* FP:context.rs-1815 */             canonical_param_env_cache: Default::default(),
/* FP:context.rs-1816 */             highest_var_in_clauses_cache: Default::default(),
/* FP:context.rs-1817 */             clauses_cache: Default::default(),
/* FP:context.rs-1818 */             data_layout,
/* FP:context.rs-1819 */             alloc_map: interpret::AllocMap::new(),
/* FP:context.rs-1820 */             current_gcx,
/* FP:context.rs-1821 */             jobserver_proxy,
/* FP:context.rs-1822 */         });
/* FP:context.rs-1823 */ 
/* FP:context.rs-1824 */         // This is a separate function to work around a crash with parallel rustc (#135870)
/* FP:context.rs-1825 */         gcx.enter(f)
/* FP:context.rs-1826 */     }
/* FP:context.rs-1827 */ 
/* FP:context.rs-1828 */     /// Obtain all lang items of this crate and all dependencies (recursively)
/* FP:context.rs-1829 */     pub fn lang_items(self) -> &'tcx crate::rustc_hir::lang_items::LanguageItems {
/* FP:context.rs-1830 */         self.get_lang_items(())
/* FP:context.rs-1831 */     }
/* FP:context.rs-1832 */ 
/* FP:context.rs-1833 */     /// Gets a `Ty` representing the [`LangItem::OrderingEnum`]
/* FP:context.rs-1834 */     #[track_caller]
/* FP:context.rs-1835 */     pub fn ty_ordering_enum(self, span: Span) -> Ty<'tcx> {
/* FP:context.rs-1836 */         let ordering_enum = self.require_lang_item(hir::LangItem::OrderingEnum, span);
/* FP:context.rs-1837 */         self.type_of(ordering_enum).no_bound_vars().unwrap()
/* FP:context.rs-1838 */     }
/* FP:context.rs-1839 */ 
/* FP:context.rs-1840 */     /// Obtain the given diagnostic item's `DefId`. Use `is_diagnostic_item` if you just want to
/* FP:context.rs-1841 */     /// compare against another `DefId`, since `is_diagnostic_item` is cheaper.
/* FP:context.rs-1842 */     pub fn get_diagnostic_item(self, name: Symbol) -> Option<DefId> {
/* FP:context.rs-1843 */         self.all_diagnostic_items(()).name_to_id.get(&name).copied()
/* FP:context.rs-1844 */     }
/* FP:context.rs-1845 */ 
/* FP:context.rs-1846 */     /// Obtain the diagnostic item's name
/* FP:context.rs-1847 */     pub fn get_diagnostic_name(self, id: DefId) -> Option<Symbol> {
/* FP:context.rs-1848 */         self.diagnostic_items(id.krate).id_to_name.get(&id).copied()
/* FP:context.rs-1849 */     }
/* FP:context.rs-1850 */ 
/* FP:context.rs-1851 */     /// Check whether the diagnostic item with the given `name` has the given `DefId`.
/* FP:context.rs-1852 */     pub fn is_diagnostic_item(self, name: Symbol, did: DefId) -> bool {
/* FP:context.rs-1853 */         self.diagnostic_items(did.krate).name_to_id.get(&name) == Some(&did)
/* FP:context.rs-1854 */     }
/* FP:context.rs-1855 */ 
/* FP:context.rs-1856 */     pub fn is_coroutine(self, def_id: DefId) -> bool {
/* FP:context.rs-1857 */         self.coroutine_kind(def_id).is_some()
/* FP:context.rs-1858 */     }
/* FP:context.rs-1859 */ 
/* FP:context.rs-1860 */     pub fn is_async_drop_in_place_coroutine(self, def_id: DefId) -> bool {
/* FP:context.rs-1861 */         self.is_lang_item(self.parent(def_id), LangItem::AsyncDropInPlace)
/* FP:context.rs-1862 */     }
/* FP:context.rs-1863 */ 
/* FP:context.rs-1864 */     /// Returns the movability of the coroutine of `def_id`, or panics
/* FP:context.rs-1865 */     /// if given a `def_id` that is not a coroutine.
/* FP:context.rs-1866 */     pub fn coroutine_movability(self, def_id: DefId) -> hir::Movability {
/* FP:context.rs-1867 */         self.coroutine_kind(def_id).expect("expected a coroutine").movability()
/* FP:context.rs-1868 */     }
/* FP:context.rs-1869 */ 
/* FP:context.rs-1870 */     /// Returns `true` if the node pointed to by `def_id` is a coroutine for an async construct.
/* FP:context.rs-1871 */     pub fn coroutine_is_async(self, def_id: DefId) -> bool {
/* FP:context.rs-1872 */         matches!(
/* FP:context.rs-1873 */             self.coroutine_kind(def_id),
/* FP:context.rs-1874 */             Some(hir::CoroutineKind::Desugared(hir::CoroutineDesugaring::Async, _))
/* FP:context.rs-1875 */         )
/* FP:context.rs-1876 */     }
/* FP:context.rs-1877 */ 
/* FP:context.rs-1878 */     // Whether the body owner is synthetic, which in this case means it does not correspond to
/* FP:context.rs-1879 */     // meaningful HIR. This is currently used to skip over MIR borrowck.
/* FP:context.rs-1880 */     pub fn is_synthetic_mir(self, def_id: impl Into<DefId>) -> bool {
/* FP:context.rs-1881 */         matches!(self.def_kind(def_id.into()), DefKind::SyntheticCoroutineBody)
/* FP:context.rs-1882 */     }
/* FP:context.rs-1883 */ 
/* FP:context.rs-1884 */     /// Returns `true` if the node pointed to by `def_id` is a general coroutine that implements `Coroutine`.
/* FP:context.rs-1885 */     /// This means it is neither an `async` or `gen` construct.
/* FP:context.rs-1886 */     pub fn is_general_coroutine(self, def_id: DefId) -> bool {
/* FP:context.rs-1887 */         matches!(self.coroutine_kind(def_id), Some(hir::CoroutineKind::Coroutine(_)))
/* FP:context.rs-1888 */     }
/* FP:context.rs-1889 */ 
/* FP:context.rs-1890 */     /// Returns `true` if the node pointed to by `def_id` is a coroutine for a `gen` construct.
/* FP:context.rs-1891 */     pub fn coroutine_is_gen(self, def_id: DefId) -> bool {
/* FP:context.rs-1892 */         matches!(
/* FP:context.rs-1893 */             self.coroutine_kind(def_id),
/* FP:context.rs-1894 */             Some(hir::CoroutineKind::Desugared(hir::CoroutineDesugaring::Gen, _))
/* FP:context.rs-1895 */         )
/* FP:context.rs-1896 */     }
/* FP:context.rs-1897 */ 
/* FP:context.rs-1898 */     /// Returns `true` if the node pointed to by `def_id` is a coroutine for a `async gen` construct.
/* FP:context.rs-1899 */     pub fn coroutine_is_async_gen(self, def_id: DefId) -> bool {
/* FP:context.rs-1900 */         matches!(
/* FP:context.rs-1901 */             self.coroutine_kind(def_id),
/* FP:context.rs-1902 */             Some(hir::CoroutineKind::Desugared(hir::CoroutineDesugaring::AsyncGen, _))
/* FP:context.rs-1903 */         )
/* FP:context.rs-1904 */     }
/* FP:context.rs-1905 */ 
/* FP:context.rs-1906 */     pub fn features(self) -> &'tcx crate::rustc_feature::Features {
/* FP:context.rs-1907 */         self.features_query(())
/* FP:context.rs-1908 */     }
/* FP:context.rs-1909 */ 
/* FP:context.rs-1910 */     pub fn def_key(self, id: impl IntoQueryParam<DefId>) -> crate::rustc_hir::definitions::DefKey {
/* FP:context.rs-1911 */         let id = id.into_query_param();
/* FP:context.rs-1912 */         // Accessing the DefKey is ok, since it is part of DefPathHash.
/* FP:context.rs-1913 */         if let Some(id) = id.as_local() {
/* FP:context.rs-1914 */             self.definitions_untracked().def_key(id)
/* FP:context.rs-1915 */         } else {
/* FP:context.rs-1916 */             self.cstore_untracked().def_key(id)
/* FP:context.rs-1917 */         }
/* FP:context.rs-1918 */     }
/* FP:context.rs-1919 */ 
/* FP:context.rs-1920 */     /// Converts a `DefId` into its fully expanded `DefPath` (every
/* FP:context.rs-1921 */     /// `DefId` is really just an interned `DefPath`).
/* FP:context.rs-1922 */     ///
/* FP:context.rs-1923 */     /// Note that if `id` is not local to this crate, the result will
/* FP:context.rs-1924 */     ///  be a non-local `DefPath`.
/* FP:context.rs-1925 */     pub fn def_path(self, id: DefId) -> crate::rustc_hir::definitions::DefPath {
/* FP:context.rs-1926 */         // Accessing the DefPath is ok, since it is part of DefPathHash.
/* FP:context.rs-1927 */         if let Some(id) = id.as_local() {
/* FP:context.rs-1928 */             self.definitions_untracked().def_path(id)
/* FP:context.rs-1929 */         } else {
/* FP:context.rs-1930 */             self.cstore_untracked().def_path(id)
/* FP:context.rs-1931 */         }
/* FP:context.rs-1932 */     }
/* FP:context.rs-1933 */ 
/* FP:context.rs-1934 */     #[inline]
/* FP:context.rs-1935 */     pub fn def_path_hash(self, def_id: DefId) -> crate::rustc_hir::definitions::DefPathHash {
/* FP:context.rs-1936 */         // Accessing the DefPathHash is ok, it is incr. comp. stable.
/* FP:context.rs-1937 */         if let Some(def_id) = def_id.as_local() {
/* FP:context.rs-1938 */             self.definitions_untracked().def_path_hash(def_id)
/* FP:context.rs-1939 */         } else {
/* FP:context.rs-1940 */             self.cstore_untracked().def_path_hash(def_id)
/* FP:context.rs-1941 */         }
/* FP:context.rs-1942 */     }
/* FP:context.rs-1943 */ 
/* FP:context.rs-1944 */     #[inline]
/* FP:context.rs-1945 */     pub fn crate_types(self) -> &'tcx [CrateType] {
/* FP:context.rs-1946 */         &self.crate_types
/* FP:context.rs-1947 */     }
/* FP:context.rs-1948 */ 
/* FP:context.rs-1949 */     pub fn needs_metadata(self) -> bool {
/* FP:context.rs-1950 */         self.crate_types().iter().any(|ty| match *ty {
/* FP:context.rs-1951 */             CrateType::Executable
/* FP:context.rs-1952 */             | CrateType::Staticlib
/* FP:context.rs-1953 */             | CrateType::Cdylib
/* FP:context.rs-1954 */             | CrateType::Sdylib => false,
/* FP:context.rs-1955 */             CrateType::Rlib | CrateType::Dylib | CrateType::ProcMacro => true,
/* FP:context.rs-1956 */         })
/* FP:context.rs-1957 */     }
/* FP:context.rs-1958 */ 
/* FP:context.rs-1959 */     pub fn needs_crate_hash(self) -> bool {
/* FP:context.rs-1960 */         // Why is the crate hash needed for these configurations?
/* FP:context.rs-1961 */         // - debug_assertions: for the "fingerprint the result" check in
/* FP:context.rs-1962 */         //   `rustc_query_system::query::plumbing::execute_job`.
/* FP:context.rs-1963 */         // - incremental: for query lookups.
/* FP:context.rs-1964 */         // - needs_metadata: for putting into crate metadata.
/* FP:context.rs-1965 */         // - instrument_coverage: for putting into coverage data (see
/* FP:context.rs-1966 */         //   `hash_mir_source`).
/* FP:context.rs-1967 */         // - metrics_dir: metrics use the strict version hash in the filenames
/* FP:context.rs-1968 */         //   for dumped metrics files to prevent overwriting distinct metrics
/* FP:context.rs-1969 */         //   for similar source builds (may change in the future, this is part
/* FP:context.rs-1970 */         //   of the proof of concept impl for the metrics initiative project goal)
/* FP:context.rs-1971 */         cfg!(debug_assertions)
/* FP:context.rs-1972 */             || self.sess.opts.incremental.is_some()
/* FP:context.rs-1973 */             || self.needs_metadata()
/* FP:context.rs-1974 */             || self.sess.instrument_coverage()
/* FP:context.rs-1975 */             || self.sess.opts.unstable_opts.metrics_dir.is_some()
/* FP:context.rs-1976 */     }
/* FP:context.rs-1977 */ 
/* FP:context.rs-1978 */     #[inline]
/* FP:context.rs-1979 */     pub fn stable_crate_id(self, crate_num: CrateNum) -> StableCrateId {
/* FP:context.rs-1980 */         if crate_num == LOCAL_CRATE {
/* FP:context.rs-1981 */             self.stable_crate_id
/* FP:context.rs-1982 */         } else {
/* FP:context.rs-1983 */             self.cstore_untracked().stable_crate_id(crate_num)
/* FP:context.rs-1984 */         }
/* FP:context.rs-1985 */     }
/* FP:context.rs-1986 */ 
/* FP:context.rs-1987 */     /// Maps a StableCrateId to the corresponding CrateNum. This method assumes
/* FP:context.rs-1988 */     /// that the crate in question has already been loaded by the CrateStore.
/* FP:context.rs-1989 */     #[inline]
/* FP:context.rs-1990 */     pub fn stable_crate_id_to_crate_num(self, stable_crate_id: StableCrateId) -> CrateNum {
/* FP:context.rs-1991 */         if stable_crate_id == self.stable_crate_id(LOCAL_CRATE) {
/* FP:context.rs-1992 */             LOCAL_CRATE
/* FP:context.rs-1993 */         } else {
/* FP:context.rs-1994 */             *self
/* FP:context.rs-1995 */                 .untracked()
/* FP:context.rs-1996 */                 .stable_crate_ids
/* FP:context.rs-1997 */                 .read()
/* FP:context.rs-1998 */                 .get(&stable_crate_id)
/* FP:context.rs-1999 */                 .unwrap_or_else(|| bug!("uninterned StableCrateId: {stable_crate_id:?}"))
/* FP:context.rs-2000 */         }
/* FP:context.rs-2001 */     }
/* FP:context.rs-2002 */ 
/* FP:context.rs-2003 */     /// Converts a `DefPathHash` to its corresponding `DefId` in the current compilation
/* FP:context.rs-2004 */     /// session, if it still exists. This is used during incremental compilation to
/* FP:context.rs-2005 */     /// turn a deserialized `DefPathHash` into its current `DefId`.
/* FP:context.rs-2006 */     pub fn def_path_hash_to_def_id(self, hash: DefPathHash) -> Option<DefId> {
/* FP:context.rs-2007 */         debug!("def_path_hash_to_def_id({:?})", hash);
/* FP:context.rs-2008 */ 
/* FP:context.rs-2009 */         let stable_crate_id = hash.stable_crate_id();
/* FP:context.rs-2010 */ 
/* FP:context.rs-2011 */         // If this is a DefPathHash from the local crate, we can look up the
/* FP:context.rs-2012 */         // DefId in the tcx's `Definitions`.
/* FP:context.rs-2013 */         if stable_crate_id == self.stable_crate_id(LOCAL_CRATE) {
/* FP:context.rs-2014 */             Some(self.untracked.definitions.read().local_def_path_hash_to_def_id(hash)?.to_def_id())
/* FP:context.rs-2015 */         } else {
/* FP:context.rs-2016 */             Some(self.def_path_hash_to_def_id_extern(hash, stable_crate_id))
/* FP:context.rs-2017 */         }
/* FP:context.rs-2018 */     }
/* FP:context.rs-2019 */ 
/* FP:context.rs-2020 */     pub fn def_path_debug_str(self, def_id: DefId) -> String {
/* FP:context.rs-2021 */         // We are explicitly not going through queries here in order to get
/* FP:context.rs-2022 */         // crate name and stable crate id since this code is called from debug!()
/* FP:context.rs-2023 */         // statements within the query system and we'd run into endless
/* FP:context.rs-2024 */         // recursion otherwise.
/* FP:context.rs-2025 */         let (crate_name, stable_crate_id) = if def_id.is_local() {
/* FP:context.rs-2026 */             (self.crate_name(LOCAL_CRATE), self.stable_crate_id(LOCAL_CRATE))
/* FP:context.rs-2027 */         } else {
/* FP:context.rs-2028 */             let cstore = &*self.cstore_untracked();
/* FP:context.rs-2029 */             (cstore.crate_name(def_id.krate), cstore.stable_crate_id(def_id.krate))
/* FP:context.rs-2030 */         };
/* FP:context.rs-2031 */ 
/* FP:context.rs-2032 */         format!(
/* FP:context.rs-2033 */             "{}[{:04x}]{}",
/* FP:context.rs-2034 */             crate_name,
/* FP:context.rs-2035 */             // Don't print the whole stable crate id. That's just
/* FP:context.rs-2036 */             // annoying in debug output.
/* FP:context.rs-2037 */             stable_crate_id.as_u64() >> (8 * 6),
/* FP:context.rs-2038 */             self.def_path(def_id).to_string_no_crate_verbose()
/* FP:context.rs-2039 */         )
/* FP:context.rs-2040 */     }
/* FP:context.rs-2041 */ 
/* FP:context.rs-2042 */     pub fn dcx(self) -> DiagCtxtHandle<'tcx> {
/* FP:context.rs-2043 */         self.sess.dcx()
/* FP:context.rs-2044 */     }
/* FP:context.rs-2045 */ 
/* FP:context.rs-2046 */     pub fn is_target_feature_call_safe(
/* FP:context.rs-2047 */         self,
/* FP:context.rs-2048 */         callee_features: &[TargetFeature],
/* FP:context.rs-2049 */         body_features: &[TargetFeature],
/* FP:context.rs-2050 */     ) -> bool {
/* FP:context.rs-2051 */         // If the called function has target features the calling function hasn't,
/* FP:context.rs-2052 */         // the call requires `unsafe`. Don't check this on wasm
/* FP:context.rs-2053 */         // targets, though. For more information on wasm see the
/* FP:context.rs-2054 */         // is_like_wasm check in hir_analysis/src/collect.rs
/* FP:context.rs-2055 */         self.sess.target.options.is_like_wasm
/* FP:context.rs-2056 */             || callee_features
/* FP:context.rs-2057 */                 .iter()
/* FP:context.rs-2058 */                 .all(|feature| body_features.iter().any(|f| f.name == feature.name))
/* FP:context.rs-2059 */     }
/* FP:context.rs-2060 */ 
/* FP:context.rs-2061 */     /// Returns the safe version of the signature of the given function, if calling it
/* FP:context.rs-2062 */     /// would be safe in the context of the given caller.
/* FP:context.rs-2063 */     pub fn adjust_target_feature_sig(
/* FP:context.rs-2064 */         self,
/* FP:context.rs-2065 */         fun_def: DefId,
/* FP:context.rs-2066 */         fun_sig: ty::Binder<'tcx, ty::FnSig<'tcx>>,
/* FP:context.rs-2067 */         caller: DefId,
/* FP:context.rs-2068 */     ) -> Option<ty::Binder<'tcx, ty::FnSig<'tcx>>> {
/* FP:context.rs-2069 */         let fun_features = &self.codegen_fn_attrs(fun_def).target_features;
/* FP:context.rs-2070 */         let callee_features = &self.codegen_fn_attrs(caller).target_features;
/* FP:context.rs-2071 */         if self.is_target_feature_call_safe(&fun_features, &callee_features) {
/* FP:context.rs-2072 */             return Some(fun_sig.map_bound(|sig| ty::FnSig { safety: hir::Safety::Safe, ..sig }));
/* FP:context.rs-2073 */         }
/* FP:context.rs-2074 */         None
/* FP:context.rs-2075 */     }
/* FP:context.rs-2076 */ 
/* FP:context.rs-2077 */     /// Helper to get a tracked environment variable via. [`TyCtxt::env_var_os`] and converting to
/* FP:context.rs-2078 */     /// UTF-8 like [`std::env::var`].
/* FP:context.rs-2079 */     pub fn env_var<K: ?Sized + AsRef<OsStr>>(self, key: &'tcx K) -> Result<&'tcx str, VarError> {
/* FP:context.rs-2080 */         match self.env_var_os(key.as_ref()) {
/* FP:context.rs-2081 */             Some(value) => value.to_str().ok_or_else(|| VarError::NotUnicode(value.to_os_string())),
/* FP:context.rs-2082 */             None => Err(VarError::NotPresent),
/* FP:context.rs-2083 */         }
/* FP:context.rs-2084 */     }
/* FP:context.rs-2085 */ }
/* FP:context.rs-2086 */ 
/* FP:context.rs-2087 */ impl<'tcx> TyCtxtAt<'tcx> {
/* FP:context.rs-2088 */     /// Create a new definition within the incr. comp. engine.
/* FP:context.rs-2089 */     pub fn create_def(
/* FP:context.rs-2090 */         self,
/* FP:context.rs-2091 */         parent: LocalDefId,
/* FP:context.rs-2092 */         name: Option<Symbol>,
/* FP:context.rs-2093 */         def_kind: DefKind,
/* FP:context.rs-2094 */         override_def_path_data: Option<DefPathData>,
/* FP:context.rs-2095 */         disambiguator: &mut DisambiguatorState,
/* FP:context.rs-2096 */     ) -> TyCtxtFeed<'tcx, LocalDefId> {
/* FP:context.rs-2097 */         let feed =
/* FP:context.rs-2098 */             self.tcx.create_def(parent, name, def_kind, override_def_path_data, disambiguator);
/* FP:context.rs-2099 */ 
/* FP:context.rs-2100 */         feed.def_span(self.span);
/* FP:context.rs-2101 */         feed
/* FP:context.rs-2102 */     }
/* FP:context.rs-2103 */ }
/* FP:context.rs-2104 */ 
/* FP:context.rs-2105 */ impl<'tcx> TyCtxt<'tcx> {
/* FP:context.rs-2106 */     /// `tcx`-dependent operations performed for every created definition.
/* FP:context.rs-2107 */     pub fn create_def(
/* FP:context.rs-2108 */         self,
/* FP:context.rs-2109 */         parent: LocalDefId,
/* FP:context.rs-2110 */         name: Option<Symbol>,
/* FP:context.rs-2111 */         def_kind: DefKind,
/* FP:context.rs-2112 */         override_def_path_data: Option<DefPathData>,
/* FP:context.rs-2113 */         disambiguator: &mut DisambiguatorState,
/* FP:context.rs-2114 */     ) -> TyCtxtFeed<'tcx, LocalDefId> {
/* FP:context.rs-2115 */         let data = override_def_path_data.unwrap_or_else(|| def_kind.def_path_data(name));
/* FP:context.rs-2116 */         // The following call has the side effect of modifying the tables inside `definitions`.
/* FP:context.rs-2117 */         // These very tables are relied on by the incr. comp. engine to decode DepNodes and to
/* FP:context.rs-2118 */         // decode the on-disk cache.
/* FP:context.rs-2119 */         //
/* FP:context.rs-2120 */         // Any LocalDefId which is used within queries, either as key or result, either:
/* FP:context.rs-2121 */         // - has been created before the construction of the TyCtxt;
/* FP:context.rs-2122 */         // - has been created by this call to `create_def`.
/* FP:context.rs-2123 */         // As a consequence, this LocalDefId is always re-created before it is needed by the incr.
/* FP:context.rs-2124 */         // comp. engine itself.
/* FP:context.rs-2125 */         let def_id = self.untracked.definitions.write().create_def(parent, data, disambiguator);
/* FP:context.rs-2126 */ 
/* FP:context.rs-2127 */         // This function modifies `self.definitions` using a side-effect.
/* FP:context.rs-2128 */         // We need to ensure that these side effects are re-run by the incr. comp. engine.
/* FP:context.rs-2129 */         // Depending on the forever-red node will tell the graph that the calling query
/* FP:context.rs-2130 */         // needs to be re-evaluated.
/* FP:context.rs-2131 */         self.dep_graph.read_index(DepNodeIndex::FOREVER_RED_NODE);
/* FP:context.rs-2132 */ 
/* FP:context.rs-2133 */         let feed = TyCtxtFeed { tcx: self, key: def_id };
/* FP:context.rs-2134 */         feed.def_kind(def_kind);
/* FP:context.rs-2135 */         // Unique types created for closures participate in type privacy checking.
/* FP:context.rs-2136 */         // They have visibilities inherited from the module they are defined in.
/* FP:context.rs-2137 */         // Visibilities for opaque types are meaningless, but still provided
/* FP:context.rs-2138 */         // so that all items have visibilities.
/* FP:context.rs-2139 */         if matches!(def_kind, DefKind::Closure | DefKind::OpaqueTy) {
/* FP:context.rs-2141 */             feed.visibility(ty::Visibility::Restricted(parent_mod));
/* FP:context.rs-2142 */         }
/* FP:context.rs-2143 */ 
/* FP:context.rs-2144 */         feed
/* FP:context.rs-2145 */     }
/* FP:context.rs-2146 */ 
/* FP:context.rs-2147 */     pub fn create_crate_num(
/* FP:context.rs-2148 */         self,
/* FP:context.rs-2149 */         stable_crate_id: StableCrateId,
/* FP:context.rs-2150 */     ) -> Result<TyCtxtFeed<'tcx, CrateNum>, CrateNum> {
/* FP:context.rs-2151 */         if let Some(&existing) = self.untracked().stable_crate_ids.read().get(&stable_crate_id) {
/* FP:context.rs-2152 */             return Err(existing);
/* FP:context.rs-2153 */         }
/* FP:context.rs-2154 */ 
/* FP:context.rs-2155 */         let num = CrateNum::new(self.untracked().stable_crate_ids.read().len());
/* FP:context.rs-2156 */         self.untracked().stable_crate_ids.write().insert(stable_crate_id, num);
/* FP:context.rs-2157 */         Ok(TyCtxtFeed { key: num, tcx: self })
/* FP:context.rs-2158 */     }
/* FP:context.rs-2159 */ 
/* FP:context.rs-2160 */     pub fn iter_local_def_id(self) -> impl Iterator<Item = LocalDefId> {
/* FP:context.rs-2161 */         // Depend on the `analysis` query to ensure compilation if finished.
/* FP:context.rs-2162 */         self.ensure_ok().analysis(());
/* FP:context.rs-2163 */ 
/* FP:context.rs-2164 */         let definitions = &self.untracked.definitions;
/* FP:context.rs-2165 */         gen {
/* FP:context.rs-2166 */             let mut i = 0;
/* FP:context.rs-2167 */ 
/* FP:context.rs-2168 */             // Recompute the number of definitions each time, because our caller may be creating
/* FP:context.rs-2169 */             // new ones.
/* FP:context.rs-2170 */             while i < { definitions.read().num_definitions() } {
/* FP:context.rs-2171 */                 let local_def_index = crate::rustc_span::def_id::DefIndex::from_usize(i);
/* FP:context.rs-2172 */                 yield LocalDefId { local_def_index };
/* FP:context.rs-2173 */                 i += 1;
/* FP:context.rs-2174 */             }
/* FP:context.rs-2175 */ 
/* FP:context.rs-2176 */             // Freeze definitions once we finish iterating on them, to prevent adding new ones.
/* FP:context.rs-2177 */             definitions.freeze();
/* FP:context.rs-2178 */         }
/* FP:context.rs-2179 */     }
/* FP:context.rs-2180 */ 
/* FP:context.rs-2181 */     pub fn def_path_table(self) -> &'tcx crate::rustc_hir::definitions::DefPathTable {
/* FP:context.rs-2182 */         // Depend on the `analysis` query to ensure compilation if finished.
/* FP:context.rs-2183 */         self.ensure_ok().analysis(());
/* FP:context.rs-2184 */ 
/* FP:context.rs-2185 */         // Freeze definitions once we start iterating on them, to prevent adding new ones
/* FP:context.rs-2186 */         // while iterating. If some query needs to add definitions, it should be `ensure`d above.
/* FP:context.rs-2187 */         self.untracked.definitions.freeze().def_path_table()
/* FP:context.rs-2188 */     }
/* FP:context.rs-2189 */ 
/* FP:context.rs-2190 */     pub fn def_path_hash_to_def_index_map(
/* FP:context.rs-2191 */         self,
/* FP:context.rs-2192 */     ) -> &'tcx crate::rustc_hir::def_path_hash_map::DefPathHashMap {
/* FP:context.rs-2193 */         // Create a dependency to the crate to be sure we re-execute this when the amount of
/* FP:context.rs-2194 */         // definitions change.
/* FP:context.rs-2195 */         self.ensure_ok().hir_crate_items(());
/* FP:context.rs-2196 */         // Freeze definitions once we start iterating on them, to prevent adding new ones
/* FP:context.rs-2197 */         // while iterating. If some query needs to add definitions, it should be `ensure`d above.
/* FP:context.rs-2198 */         self.untracked.definitions.freeze().def_path_hash_to_def_index_map()
/* FP:context.rs-2199 */     }
/* FP:context.rs-2200 */ 
/* FP:context.rs-2201 */     /// Note that this is *untracked* and should only be used within the query
/* FP:context.rs-2202 */     /// system if the result is otherwise tracked through queries
/* FP:context.rs-2203 */     #[inline]
/* FP:context.rs-2204 */     pub fn cstore_untracked(self) -> FreezeReadGuard<'tcx, CrateStoreDyn> {
/* FP:context.rs-2205 */         FreezeReadGuard::map(self.untracked.cstore.read(), |c| &**c)
/* FP:context.rs-2206 */     }
/* FP:context.rs-2207 */ 
/* FP:context.rs-2208 */     /// Give out access to the untracked data without any sanity checks.
/* FP:context.rs-2209 */     pub fn untracked(self) -> &'tcx Untracked {
/* FP:context.rs-2210 */         &self.untracked
/* FP:context.rs-2211 */     }
/* FP:context.rs-2212 */     /// Note that this is *untracked* and should only be used within the query
/* FP:context.rs-2213 */     /// system if the result is otherwise tracked through queries
/* FP:context.rs-2214 */     #[inline]
/* FP:context.rs-2215 */     pub fn definitions_untracked(self) -> FreezeReadGuard<'tcx, Definitions> {
/* FP:context.rs-2216 */         self.untracked.definitions.read()
/* FP:context.rs-2217 */     }
/* FP:context.rs-2218 */ 
/* FP:context.rs-2219 */     /// Note that this is *untracked* and should only be used within the query
/* FP:context.rs-2220 */     /// system if the result is otherwise tracked through queries
/* FP:context.rs-2221 */     #[inline]
/* FP:context.rs-2222 */     pub fn source_span_untracked(self, def_id: LocalDefId) -> Span {
/* FP:context.rs-2223 */         self.untracked.source_span.get(def_id).unwrap_or(DUMMY_SP)
/* FP:context.rs-2224 */     }
/* FP:context.rs-2225 */ 
/* FP:context.rs-2226 */     #[inline(always)]
/* FP:context.rs-2227 */     pub fn with_stable_hashing_context<R>(
/* FP:context.rs-2228 */         self,
/* FP:context.rs-2229 */         f: impl FnOnce(StableHashingContext<'_>) -> R,
/* FP:context.rs-2230 */     ) -> R {
/* FP:context.rs-2231 */         f(StableHashingContext::new(self.sess, &self.untracked))
/* FP:context.rs-2232 */     }
/* FP:context.rs-2233 */ 
/* FP:context.rs-2234 */     pub fn serialize_query_result_cache(self, encoder: FileEncoder) -> FileEncodeResult {
/* FP:context.rs-2235 */         self.query_system.on_disk_cache.as_ref().map_or(Ok(0), |c| c.serialize(self, encoder))
/* FP:context.rs-2236 */     }
/* FP:context.rs-2237 */ 
/* FP:context.rs-2238 */     #[inline]
/* FP:context.rs-2239 */     pub fn local_crate_exports_generics(self) -> bool {
/* FP:context.rs-2240 */         self.crate_types().iter().any(|crate_type| {
/* FP:context.rs-2241 */             match crate_type {
/* FP:context.rs-2242 */                 CrateType::Executable
/* FP:context.rs-2243 */                 | CrateType::Staticlib
/* FP:context.rs-2244 */                 | CrateType::ProcMacro
/* FP:context.rs-2245 */                 | CrateType::Cdylib
/* FP:context.rs-2246 */                 | CrateType::Sdylib => false,
/* FP:context.rs-2247 */ 
/* FP:context.rs-2248 */                 // FIXME rust-lang/rust#64319, rust-lang/rust#64872:
/* FP:context.rs-2249 */                 // We want to block export of generics from dylibs,
/* FP:context.rs-2250 */                 // but we must fix rust-lang/rust#65890 before we can
/* FP:context.rs-2251 */                 // do that robustly.
/* FP:context.rs-2252 */                 CrateType::Dylib => true,
/* FP:context.rs-2253 */ 
/* FP:context.rs-2254 */                 CrateType::Rlib => true,
/* FP:context.rs-2255 */             }
/* FP:context.rs-2256 */         })
/* FP:context.rs-2257 */     }
/* FP:context.rs-2258 */ 
/* FP:context.rs-2259 */     /// Returns the `DefId` and the `BoundRegionKind` corresponding to the given region.
/* FP:context.rs-2260 */     pub fn is_suitable_region(
/* FP:context.rs-2261 */         self,
/* FP:context.rs-2262 */         generic_param_scope: LocalDefId,
/* FP:context.rs-2263 */         mut region: Region<'tcx>,
/* FP:context.rs-2264 */     ) -> Option<FreeRegionInfo> {
/* FP:context.rs-2265 */         let (suitable_region_binding_scope, region_def_id) = loop {
/* FP:context.rs-2266 */             let def_id =
/* FP:context.rs-2267 */                 region.opt_param_def_id(self, generic_param_scope.to_def_id())?.as_local()?;
/* FP:context.rs-2268 */             let scope = self.local_parent(def_id);
/* FP:context.rs-2269 */             if self.def_kind(scope) == DefKind::OpaqueTy {
/* FP:context.rs-2270 */                 // Lifetime params of opaque types are synthetic and thus irrelevant to
/* FP:context.rs-2271 */                 // diagnostics. Map them back to their origin!
/* FP:context.rs-2272 */                 region = self.map_opaque_lifetime_to_parent_lifetime(def_id);
/* FP:context.rs-2273 */                 continue;
/* FP:context.rs-2274 */             }
/* FP:context.rs-2275 */             break (scope, def_id.into());
/* FP:context.rs-2276 */         };
/* FP:context.rs-2277 */ 
/* FP:context.rs-2278 */         let is_impl_item = match self.hir_node_by_def_id(suitable_region_binding_scope) {
/* FP:context.rs-2279 */             Node::Item(..) | Node::TraitItem(..) => false,
/* FP:context.rs-2280 */             Node::ImplItem(impl_item) => match impl_item.impl_kind {
/* FP:context.rs-2281 */                 // For now, we do not try to target impls of traits. This is
/* FP:context.rs-2282 */                 // because this message is going to suggest that the user
/* FP:context.rs-2283 */                 // change the fn signature, but they may not be free to do so,
/* FP:context.rs-2284 */                 // since the signature must match the trait.
/* FP:context.rs-2285 */                 //
/* FP:context.rs-2286 */                 // FIXME(#42706) -- in some cases, we could do better here.
/* FP:context.rs-2287 */                 hir::ImplItemImplKind::Trait { .. } => true,
/* FP:context.rs-2288 */                 _ => false,
/* FP:context.rs-2289 */             },
/* FP:context.rs-2290 */             _ => false,
/* FP:context.rs-2291 */         };
/* FP:context.rs-2292 */ 
/* FP:context.rs-2293 */         Some(FreeRegionInfo { scope: suitable_region_binding_scope, region_def_id, is_impl_item })
/* FP:context.rs-2294 */     }
/* FP:context.rs-2295 */ 
/* FP:context.rs-2296 */     /// Given a `DefId` for an `fn`, return all the `dyn` and `impl` traits in its return type.
/* FP:context.rs-2297 */     pub fn return_type_impl_or_dyn_traits(
/* FP:context.rs-2298 */         self,
/* FP:context.rs-2299 */         scope_def_id: LocalDefId,
/* FP:context.rs-2300 */     ) -> Vec<&'tcx hir::Ty<'tcx>> {
/* FP:context.rs-2301 */         let hir_id = self.local_def_id_to_hir_id(scope_def_id);
/* FP:context.rs-2302 */         let Some(hir::FnDecl { output: hir::FnRetTy::Return(hir_output), .. }) =
/* FP:context.rs-2303 */             self.hir_fn_decl_by_hir_id(hir_id)
/* FP:context.rs-2304 */         else {
/* FP:context.rs-2305 */             return vec![];
/* FP:context.rs-2306 */         };
/* FP:context.rs-2307 */ 
/* FP:context.rs-2308 */         let mut v = TraitObjectVisitor(vec![]);
/* FP:context.rs-2309 */         v.visit_ty_unambig(hir_output);
/* FP:context.rs-2310 */         v.0
/* FP:context.rs-2311 */     }
/* FP:context.rs-2312 */ 
/* FP:context.rs-2313 */     /// Given a `DefId` for an `fn`, return all the `dyn` and `impl` traits in
/* FP:context.rs-2314 */     /// its return type, and the associated alias span when type alias is used,
/* FP:context.rs-2315 */     /// along with a span for lifetime suggestion (if there are existing generics).
/* FP:context.rs-2316 */     pub fn return_type_impl_or_dyn_traits_with_type_alias(
/* FP:context.rs-2317 */         self,
/* FP:context.rs-2318 */         scope_def_id: LocalDefId,
/* FP:context.rs-2319 */     ) -> Option<(Vec<&'tcx hir::Ty<'tcx>>, Span, Option<Span>)> {
/* FP:context.rs-2320 */         let hir_id = self.local_def_id_to_hir_id(scope_def_id);
/* FP:context.rs-2321 */         let mut v = TraitObjectVisitor(vec![]);
/* FP:context.rs-2322 */         // when the return type is a type alias
/* FP:context.rs-2323 */         if let Some(hir::FnDecl { output: hir::FnRetTy::Return(hir_output), .. }) = self.hir_fn_decl_by_hir_id(hir_id)
/* FP:context.rs-2324 */             && let hir::TyKind::Path(hir::QPath::Resolved(
/* FP:context.rs-2325 */                 None,
/* FP:context.rs-2326 */                 hir::Path { res: hir::def::Res::Def(DefKind::TyAlias, def_id), .. }, )) = hir_output.kind
/* FP:context.rs-2327 */             && let Some(local_id) = def_id.as_local()
/* FP:context.rs-2328 */             && let Some(alias_ty) = self.hir_node_by_def_id(local_id).alias_ty() // it is type alias
/* FP:context.rs-2329 */             && let Some(alias_generics) = self.hir_node_by_def_id(local_id).generics()
/* FP:context.rs-2330 */         {
/* FP:context.rs-2331 */             v.visit_ty_unambig(alias_ty);
/* FP:context.rs-2332 */             if !v.0.is_empty() {
/* FP:context.rs-2333 */                 return Some((
/* FP:context.rs-2334 */                     v.0,
/* FP:context.rs-2335 */                     alias_generics.span,
/* FP:context.rs-2336 */                     alias_generics.span_for_lifetime_suggestion(),
/* FP:context.rs-2337 */                 ));
/* FP:context.rs-2338 */             }
/* FP:context.rs-2339 */         }
/* FP:context.rs-2340 */         None
/* FP:context.rs-2341 */     }
/* FP:context.rs-2342 */ 
/* FP:context.rs-2343 */     /// Determines whether identifiers in the assembly have strict naming rules.
/* FP:context.rs-2344 */     /// Currently, only NVPTX* targets need it.
/* FP:context.rs-2345 */     pub fn has_strict_asm_symbol_naming(self) -> bool {
/* FP:context.rs-2346 */         self.sess.target.arch.contains("nvptx")
/* FP:context.rs-2347 */     }
/* FP:context.rs-2348 */ 
/* FP:context.rs-2349 */     /// Returns `&'static core::panic::Location<'static>`.
/* FP:context.rs-2350 */     pub fn caller_location_ty(self) -> Ty<'tcx> {
/* FP:context.rs-2351 */         Ty::new_imm_ref(
/* FP:context.rs-2352 */             self,
/* FP:context.rs-2353 */             self.lifetimes.re_static,
/* FP:context.rs-2354 */             self.type_of(self.require_lang_item(LangItem::PanicLocation, DUMMY_SP))
/* FP:context.rs-2355 */                 .instantiate(self, self.mk_args(&[self.lifetimes.re_static.into()])),
/* FP:context.rs-2356 */         )
/* FP:context.rs-2357 */     }
/* FP:context.rs-2358 */ 
/* FP:context.rs-2359 */     /// Returns a displayable description and article for the given `def_id` (e.g. `("a", "struct")`).
/* FP:context.rs-2360 */     pub fn article_and_description(self, def_id: DefId) -> (&'static str, &'static str) {
/* FP:context.rs-2361 */         let kind = self.def_kind(def_id);
/* FP:context.rs-2362 */         (self.def_kind_descr_article(kind, def_id), self.def_kind_descr(kind, def_id))
/* FP:context.rs-2363 */     }
/* FP:context.rs-2364 */ 
/* FP:context.rs-2365 */     pub fn type_length_limit(self) -> Limit {
/* FP:context.rs-2366 */         self.limits(()).type_length_limit
/* FP:context.rs-2367 */     }
/* FP:context.rs-2368 */ 
/* FP:context.rs-2369 */     pub fn recursion_limit(self) -> Limit {
/* FP:context.rs-2370 */         self.limits(()).recursion_limit
/* FP:context.rs-2371 */     }
/* FP:context.rs-2372 */ 
/* FP:context.rs-2373 */     pub fn move_size_limit(self) -> Limit {
/* FP:context.rs-2374 */         self.limits(()).move_size_limit
/* FP:context.rs-2375 */     }
/* FP:context.rs-2376 */ 
/* FP:context.rs-2377 */     pub fn pattern_complexity_limit(self) -> Limit {
/* FP:context.rs-2378 */         self.limits(()).pattern_complexity_limit
/* FP:context.rs-2379 */     }
/* FP:context.rs-2380 */ 
/* FP:context.rs-2381 */     /// All traits in the crate graph, including those not visible to the user.
/* FP:context.rs-2382 */     pub fn all_traits_including_private(self) -> impl Iterator<Item = DefId> {
/* FP:context.rs-2383 */         iter::once(LOCAL_CRATE)
/* FP:context.rs-2384 */             .chain(self.crates(()).iter().copied())
/* FP:context.rs-2385 */             .flat_map(move |cnum| self.traits(cnum).iter().copied())
/* FP:context.rs-2386 */     }
/* FP:context.rs-2387 */ 
/* FP:context.rs-2388 */     /// All traits that are visible within the crate graph (i.e. excluding private dependencies).
/* FP:context.rs-2389 */     pub fn visible_traits(self) -> impl Iterator<Item = DefId> {
/* FP:context.rs-2390 */         let visible_crates =
/* FP:context.rs-2391 */             self.crates(()).iter().copied().filter(move |cnum| self.is_user_visible_dep(*cnum));
/* FP:context.rs-2392 */ 
/* FP:context.rs-2393 */         iter::once(LOCAL_CRATE)
/* FP:context.rs-2394 */             .chain(visible_crates)
/* FP:context.rs-2395 */             .flat_map(move |cnum| self.traits(cnum).iter().copied())
/* FP:context.rs-2396 */     }
/* FP:context.rs-2397 */ 
/* FP:context.rs-2398 */     #[inline]
/* FP:context.rs-2399 */     pub fn local_visibility(self, def_id: LocalDefId) -> Visibility {
/* FP:context.rs-2400 */         self.visibility(def_id).expect_local()
/* FP:context.rs-2401 */     }
/* FP:context.rs-2402 */ 
/* FP:context.rs-2403 */     /// Returns the origin of the opaque type `def_id`.
/* FP:context.rs-2404 */     #[instrument(skip(self), level = "trace", ret)]
/* FP:context.rs-2405 */     pub fn local_opaque_ty_origin(self, def_id: LocalDefId) -> hir::OpaqueTyOrigin<LocalDefId> {
/* FP:context.rs-2406 */         self.hir_expect_opaque_ty(def_id).origin
/* FP:context.rs-2407 */     }
/* FP:context.rs-2408 */ 
/* FP:context.rs-2409 */     pub fn finish(self) {
/* FP:context.rs-2410 */         // We assume that no queries are run past here. If there are new queries
/* FP:context.rs-2411 */         // after this point, they'll show up as "<unknown>" in self-profiling data.
/* FP:context.rs-2412 */         self.alloc_self_profile_query_strings();
/* FP:context.rs-2413 */ 
/* FP:context.rs-2414 */         self.save_dep_graph();
/* FP:context.rs-2415 */         self.query_key_hash_verify_all();
/* FP:context.rs-2416 */ 
/* FP:context.rs-2417 */         if let Err((path, error)) = self.dep_graph.finish_encoding() {
/* FP:context.rs-2418 */             self.sess.dcx().emit_fatal(crate::error::FailedWritingFile { path: &path, error });
/* FP:context.rs-2419 */         }
/* FP:context.rs-2420 */     }
/* FP:context.rs-2421 */ }
/* FP:context.rs-2422 */ 
/* FP:context.rs-2423 */ macro_rules! nop_lift {
/* FP:context.rs-2424 */     ($set:ident; $ty:ty => $lifted:ty) => {
/* FP:context.rs-2425 */         impl<'a, 'tcx> Lift<TyCtxt<'tcx>> for $ty {
/* FP:context.rs-2426 */             type Lifted = $lifted;
/* FP:context.rs-2427 */             fn lift_to_interner(self, tcx: TyCtxt<'tcx>) -> Option<Self::Lifted> {
/* FP:context.rs-2428 */                 // Assert that the set has the right type.
/* FP:context.rs-2429 */                 // Given an argument that has an interned type, the return type has the type of
/* FP:context.rs-2430 */                 // the corresponding interner set. This won't actually return anything, we're
/* FP:context.rs-2431 */                 // just doing this to compute said type!
/* FP:context.rs-2432 */                 fn _intern_set_ty_from_interned_ty<'tcx, Inner>(
/* FP:context.rs-2433 */                     _x: Interned<'tcx, Inner>,
/* FP:context.rs-2434 */                 ) -> InternedSet<'tcx, Inner> {
/* FP:context.rs-2435 */                     unreachable!()
/* FP:context.rs-2436 */                 }
/* FP:context.rs-2437 */                 fn _type_eq<T>(_x: &T, _y: &T) {}
/* FP:context.rs-2438 */                 fn _test<'tcx>(x: $lifted, tcx: TyCtxt<'tcx>) {
/* FP:context.rs-2439 */                     // If `x` is a newtype around an `Interned<T>`, then `interner` is an
/* FP:context.rs-2440 */                     // interner of appropriate type. (Ideally we'd also check that `x` is a
/* FP:context.rs-2441 */                     // newtype with just that one field. Not sure how to do that.)
/* FP:context.rs-2442 */                     let interner = _intern_set_ty_from_interned_ty(x.0);
/* FP:context.rs-2443 */                     // Now check that this is the same type as `interners.$set`.
/* FP:context.rs-2444 */                     _type_eq(&interner, &tcx.interners.$set);
/* FP:context.rs-2445 */                 }
/* FP:context.rs-2446 */ 
/* FP:context.rs-2447 */                 tcx.interners
/* FP:context.rs-2448 */                     .$set
/* FP:context.rs-2449 */                     .contains_pointer_to(&InternedInSet(&*self.0.0))
/* FP:context.rs-2450 */                     // SAFETY: `self` is interned and therefore valid
/* FP:context.rs-2451 */                     // for the entire lifetime of the `TyCtxt`.
/* FP:context.rs-2452 */                     .then(|| unsafe { mem::transmute(self) })
/* FP:context.rs-2453 */             }
/* FP:context.rs-2454 */         }
/* FP:context.rs-2455 */     };
/* FP:context.rs-2456 */ }
/* FP:context.rs-2457 */ 
/* FP:context.rs-2458 */ macro_rules! nop_list_lift {
/* FP:context.rs-2459 */     ($set:ident; $ty:ty => $lifted:ty) => {
/* FP:context.rs-2460 */         impl<'a, 'tcx> Lift<TyCtxt<'tcx>> for &'a List<$ty> {
/* FP:context.rs-2461 */             type Lifted = &'tcx List<$lifted>;
/* FP:context.rs-2462 */             fn lift_to_interner(self, tcx: TyCtxt<'tcx>) -> Option<Self::Lifted> {
/* FP:context.rs-2463 */                 // Assert that the set has the right type.
/* FP:context.rs-2464 */                 if false {
/* FP:context.rs-2465 */                     let _x: &InternedSet<'tcx, List<$lifted>> = &tcx.interners.$set;
/* FP:context.rs-2466 */                 }
/* FP:context.rs-2467 */ 
/* FP:context.rs-2468 */                 if self.is_empty() {
/* FP:context.rs-2469 */                     return Some(List::empty());
/* FP:context.rs-2470 */                 }
/* FP:context.rs-2471 */                 tcx.interners
/* FP:context.rs-2472 */                     .$set
/* FP:context.rs-2473 */                     .contains_pointer_to(&InternedInSet(self))
/* FP:context.rs-2474 */                     .then(|| unsafe { mem::transmute(self) })
/* FP:context.rs-2475 */             }
/* FP:context.rs-2476 */         }
/* FP:context.rs-2477 */     };
/* FP:context.rs-2478 */ }
/* FP:context.rs-2479 */ 
/* FP:context.rs-2480 */ nop_lift! { type_; Ty<'a> => Ty<'tcx> }
/* FP:context.rs-2481 */ nop_lift! { region; Region<'a> => Region<'tcx> }
/* FP:context.rs-2482 */ nop_lift! { const_; Const<'a> => Const<'tcx> }
/* FP:context.rs-2483 */ nop_lift! { pat; Pattern<'a> => Pattern<'tcx> }
/* FP:context.rs-2484 */ nop_lift! { const_allocation; ConstAllocation<'a> => ConstAllocation<'tcx> }
/* FP:context.rs-2485 */ nop_lift! { predicate; Predicate<'a> => Predicate<'tcx> }
/* FP:context.rs-2486 */ nop_lift! { predicate; Clause<'a> => Clause<'tcx> }
/* FP:context.rs-2487 */ nop_lift! { layout; Layout<'a> => Layout<'tcx> }
/* FP:context.rs-2488 */ nop_lift! { valtree; ValTree<'a> => ValTree<'tcx> }
/* FP:context.rs-2489 */ 
/* FP:context.rs-2490 */ nop_list_lift! { type_lists; Ty<'a> => Ty<'tcx> }
/* FP:context.rs-2491 */ nop_list_lift! {
/* FP:context.rs-2492 */     poly_existential_predicates; PolyExistentialPredicate<'a> => PolyExistentialPredicate<'tcx>
/* FP:context.rs-2493 */ }
/* FP:context.rs-2494 */ nop_list_lift! { bound_variable_kinds; ty::BoundVariableKind => ty::BoundVariableKind }
/* FP:context.rs-2495 */ 
/* FP:context.rs-2496 */ // This is the impl for `&'a GenericArgs<'a>`.
/* FP:context.rs-2497 */ nop_list_lift! { args; GenericArg<'a> => GenericArg<'tcx> }
/* FP:context.rs-2498 */ 
/* FP:context.rs-2499 */ macro_rules! sty_debug_print {
/* FP:context.rs-2500 */     ($fmt: expr, $ctxt: expr, $($variant: ident),*) => {{
/* FP:context.rs-2501 */         // Curious inner module to allow variant names to be used as
/* FP:context.rs-2502 */         // variable names.
/* FP:context.rs-2503 */         #[allow(non_snake_case)]
/* FP:context.rs-2504 */         mod inner {
/* FP:context.rs-2505 */             use crate::ty::{self, TyCtxt};
/* FP:context.rs-2506 */             use crate::ty::context::InternedInSet;
/* FP:context.rs-2507 */ 
/* FP:context.rs-2508 */             #[derive(Copy, Clone)]
/* FP:context.rs-2509 */             struct DebugStat {
/* FP:context.rs-2510 */                 total: usize,
/* FP:context.rs-2511 */                 lt_infer: usize,
/* FP:context.rs-2512 */                 ty_infer: usize,
/* FP:context.rs-2513 */                 ct_infer: usize,
/* FP:context.rs-2514 */                 all_infer: usize,
/* FP:context.rs-2515 */             }
/* FP:context.rs-2516 */ 
/* FP:context.rs-2517 */             pub(crate) fn go(fmt: &mut std::fmt::Formatter<'_>, tcx: TyCtxt<'_>) -> std::fmt::Result {
/* FP:context.rs-2518 */                 let mut total = DebugStat {
/* FP:context.rs-2519 */                     total: 0,
/* FP:context.rs-2520 */                     lt_infer: 0,
/* FP:context.rs-2521 */                     ty_infer: 0,
/* FP:context.rs-2522 */                     ct_infer: 0,
/* FP:context.rs-2523 */                     all_infer: 0,
/* FP:context.rs-2524 */                 };
/* FP:context.rs-2525 */                 $(let mut $variant = total;)*
/* FP:context.rs-2526 */ 
/* FP:context.rs-2527 */                 for shard in tcx.interners.type_.lock_shards() {
/* FP:context.rs-2528 */                     // It seems that ordering doesn't affect anything here.
/* FP:context.rs-2529 */                     #[allow(rustc::potential_query_instability)]
/* FP:context.rs-2530 */                     let types = shard.iter();
/* FP:context.rs-2531 */                     for &(InternedInSet(t), ()) in types {
/* FP:context.rs-2532 */                         let variant = match t.internee {
/* FP:context.rs-2533 */                             ty::Bool | ty::Char | ty::Int(..) | ty::Uint(..) |
/* FP:context.rs-2534 */                                 ty::Float(..) | ty::Str | ty::Never => continue,
/* FP:context.rs-2535 */                             ty::Error(_) => /* unimportant */ continue,
/* FP:context.rs-2536 */                             $(ty::$variant(..) => &mut $variant,)*
/* FP:context.rs-2537 */                         };
/* FP:context.rs-2538 */                         let lt = t.flags.intersects(ty::TypeFlags::HAS_RE_INFER);
/* FP:context.rs-2539 */                         let ty = t.flags.intersects(ty::TypeFlags::HAS_TY_INFER);
/* FP:context.rs-2540 */                         let ct = t.flags.intersects(ty::TypeFlags::HAS_CT_INFER);
/* FP:context.rs-2541 */ 
/* FP:context.rs-2542 */                         variant.total += 1;
/* FP:context.rs-2543 */                         total.total += 1;
/* FP:context.rs-2544 */                         if lt { total.lt_infer += 1; variant.lt_infer += 1 }
/* FP:context.rs-2545 */                         if ty { total.ty_infer += 1; variant.ty_infer += 1 }
/* FP:context.rs-2546 */                         if ct { total.ct_infer += 1; variant.ct_infer += 1 }
/* FP:context.rs-2547 */                         if lt && ty && ct { total.all_infer += 1; variant.all_infer += 1 }
/* FP:context.rs-2548 */                     }
/* FP:context.rs-2549 */                 }
/* FP:context.rs-2550 */                 writeln!(fmt, "Ty interner             total           ty lt ct all")?;
/* FP:context.rs-2551 */                 $(writeln!(fmt, "    {:18}: {uses:6} {usespc:4.1}%, \
/* FP:context.rs-2552 */                             {ty:4.1}% {lt:5.1}% {ct:4.1}% {all:4.1}%",
/* FP:context.rs-2553 */                     stringify!($variant),
/* FP:context.rs-2554 */                     uses = $variant.total,
/* FP:context.rs-2555 */                     usespc = $variant.total as f64 * 100.0 / total.total as f64,
/* FP:context.rs-2556 */                     ty = $variant.ty_infer as f64 * 100.0  / total.total as f64,
/* FP:context.rs-2557 */                     lt = $variant.lt_infer as f64 * 100.0  / total.total as f64,
/* FP:context.rs-2558 */                     ct = $variant.ct_infer as f64 * 100.0  / total.total as f64,
/* FP:context.rs-2559 */                     all = $variant.all_infer as f64 * 100.0  / total.total as f64)?;
/* FP:context.rs-2560 */                 )*
/* FP:context.rs-2561 */                 writeln!(fmt, "                  total {uses:6}        \
/* FP:context.rs-2562 */                           {ty:4.1}% {lt:5.1}% {ct:4.1}% {all:4.1}%",
/* FP:context.rs-2563 */                     uses = total.total,
/* FP:context.rs-2564 */                     ty = total.ty_infer as f64 * 100.0  / total.total as f64,
/* FP:context.rs-2565 */                     lt = total.lt_infer as f64 * 100.0  / total.total as f64,
/* FP:context.rs-2566 */                     ct = total.ct_infer as f64 * 100.0  / total.total as f64,
/* FP:context.rs-2567 */                     all = total.all_infer as f64 * 100.0  / total.total as f64)
/* FP:context.rs-2568 */             }
/* FP:context.rs-2569 */         }
/* FP:context.rs-2570 */ 
/* FP:context.rs-2571 */         inner::go($fmt, $ctxt)
/* FP:context.rs-2572 */     }}
/* FP:context.rs-2573 */ }
/* FP:context.rs-2574 */ 
/* FP:context.rs-2575 */ impl<'tcx> TyCtxt<'tcx> {
/* FP:context.rs-2576 */     pub fn debug_stats(self) -> impl fmt::Debug {
/* FP:context.rs-2577 */         fmt::from_fn(move |fmt| {
/* FP:context.rs-2578 */             sty_debug_print!(
/* FP:context.rs-2579 */                 fmt,
/* FP:context.rs-2580 */                 self,
/* FP:context.rs-2581 */                 Adt,
/* FP:context.rs-2582 */                 Array,
/* FP:context.rs-2583 */                 Slice,
/* FP:context.rs-2584 */                 RawPtr,
/* FP:context.rs-2585 */                 Ref,
/* FP:context.rs-2586 */                 FnDef,
/* FP:context.rs-2587 */                 FnPtr,
/* FP:context.rs-2588 */                 UnsafeBinder,
/* FP:context.rs-2589 */                 Placeholder,
/* FP:context.rs-2590 */                 Coroutine,
/* FP:context.rs-2591 */                 CoroutineWitness,
/* FP:context.rs-2592 */                 Dynamic,
/* FP:context.rs-2593 */                 Closure,
/* FP:context.rs-2594 */                 CoroutineClosure,
/* FP:context.rs-2595 */                 Tuple,
/* FP:context.rs-2596 */                 Bound,
/* FP:context.rs-2597 */                 Param,
/* FP:context.rs-2598 */                 Infer,
/* FP:context.rs-2599 */                 Alias,
/* FP:context.rs-2600 */                 Pat,
/* FP:context.rs-2601 */                 Foreign
/* FP:context.rs-2602 */             )?;
/* FP:context.rs-2603 */ 
/* FP:context.rs-2604 */             writeln!(fmt, "GenericArgs interner: #{}", self.interners.args.len())?;
/* FP:context.rs-2605 */             writeln!(fmt, "Region interner: #{}", self.interners.region.len())?;
/* FP:context.rs-2606 */             writeln!(fmt, "Const Allocation interner: #{}", self.interners.const_allocation.len())?;
/* FP:context.rs-2607 */             writeln!(fmt, "Layout interner: #{}", self.interners.layout.len())?;
/* FP:context.rs-2608 */ 
/* FP:context.rs-2609 */             Ok(())
/* FP:context.rs-2610 */         })
/* FP:context.rs-2611 */     }
/* FP:context.rs-2612 */ }
/* FP:context.rs-2613 */ 
/* FP:context.rs-2614 */ // This type holds a `T` in the interner. The `T` is stored in the arena and
/* FP:context.rs-2615 */ // this type just holds a pointer to it, but it still effectively owns it. It
/* FP:context.rs-2616 */ // impls `Borrow` so that it can be looked up using the original
/* FP:context.rs-2617 */ // (non-arena-memory-owning) types.
/* FP:context.rs-2618 */ struct InternedInSet<'tcx, T: ?Sized + PointeeSized>(&'tcx T);
/* FP:context.rs-2619 */ 
/* FP:context.rs-2620 */ impl<'tcx, T: 'tcx + ?Sized + PointeeSized> Clone for InternedInSet<'tcx, T> {
/* FP:context.rs-2621 */     fn clone(&self) -> Self {
/* FP:context.rs-2622 */         InternedInSet(self.0)
/* FP:context.rs-2623 */     }
/* FP:context.rs-2624 */ }
/* FP:context.rs-2625 */ 
/* FP:context.rs-2626 */ impl<'tcx, T: 'tcx + ?Sized + PointeeSized> Copy for InternedInSet<'tcx, T> {}
/* FP:context.rs-2627 */ 
/* FP:context.rs-2628 */ impl<'tcx, T: 'tcx + ?Sized + PointeeSized> IntoPointer for InternedInSet<'tcx, T> {
/* FP:context.rs-2629 */     fn into_pointer(&self) -> *const () {
/* FP:context.rs-2630 */         self.0 as *const _ as *const ()
/* FP:context.rs-2631 */     }
/* FP:context.rs-2632 */ }
/* FP:context.rs-2633 */ 
/* FP:context.rs-2634 */ #[allow(rustc::usage_of_ty_tykind)]
/* FP:context.rs-2635 */ impl<'tcx, T> Borrow<T> for InternedInSet<'tcx, WithCachedTypeInfo<T>> {
/* FP:context.rs-2636 */     fn borrow(&self) -> &T {
/* FP:context.rs-2637 */         &self.0.internee
/* FP:context.rs-2638 */     }
/* FP:context.rs-2639 */ }
/* FP:context.rs-2640 */ 
/* FP:context.rs-2641 */ impl<'tcx, T: PartialEq> PartialEq for InternedInSet<'tcx, WithCachedTypeInfo<T>> {
/* FP:context.rs-2642 */     fn eq(&self, other: &InternedInSet<'tcx, WithCachedTypeInfo<T>>) -> bool {
/* FP:context.rs-2643 */         // The `Borrow` trait requires that `x.borrow() == y.borrow()` equals
/* FP:context.rs-2644 */         // `x == y`.
/* FP:context.rs-2645 */         self.0.internee == other.0.internee
/* FP:context.rs-2646 */     }
/* FP:context.rs-2647 */ }
/* FP:context.rs-2648 */ 
/* FP:context.rs-2649 */ impl<'tcx, T: Eq> Eq for InternedInSet<'tcx, WithCachedTypeInfo<T>> {}
/* FP:context.rs-2650 */ 
/* FP:context.rs-2651 */ impl<'tcx, T: Hash> Hash for InternedInSet<'tcx, WithCachedTypeInfo<T>> {
/* FP:context.rs-2652 */     fn hash<H: Hasher>(&self, s: &mut H) {
/* FP:context.rs-2653 */         // The `Borrow` trait requires that `x.borrow().hash(s) == x.hash(s)`.
/* FP:context.rs-2654 */         self.0.internee.hash(s)
/* FP:context.rs-2655 */     }
/* FP:context.rs-2656 */ }
/* FP:context.rs-2657 */ 
/* FP:context.rs-2658 */ impl<'tcx, T> Borrow<[T]> for InternedInSet<'tcx, List<T>> {
/* FP:context.rs-2659 */     fn borrow(&self) -> &[T] {
/* FP:context.rs-2660 */         &self.0[..]
/* FP:context.rs-2661 */     }
/* FP:context.rs-2662 */ }
/* FP:context.rs-2663 */ 
/* FP:context.rs-2664 */ impl<'tcx, T: PartialEq> PartialEq for InternedInSet<'tcx, List<T>> {
/* FP:context.rs-2665 */     fn eq(&self, other: &InternedInSet<'tcx, List<T>>) -> bool {
/* FP:context.rs-2666 */         // The `Borrow` trait requires that `x.borrow() == y.borrow()` equals
/* FP:context.rs-2667 */         // `x == y`.
/* FP:context.rs-2668 */         self.0[..] == other.0[..]
/* FP:context.rs-2669 */     }
/* FP:context.rs-2670 */ }
/* FP:context.rs-2671 */ 
/* FP:context.rs-2672 */ impl<'tcx, T: Eq> Eq for InternedInSet<'tcx, List<T>> {}
/* FP:context.rs-2673 */ 
/* FP:context.rs-2674 */ impl<'tcx, T: Hash> Hash for InternedInSet<'tcx, List<T>> {
/* FP:context.rs-2675 */     fn hash<H: Hasher>(&self, s: &mut H) {
/* FP:context.rs-2676 */         // The `Borrow` trait requires that `x.borrow().hash(s) == x.hash(s)`.
/* FP:context.rs-2677 */         self.0[..].hash(s)
/* FP:context.rs-2678 */     }
/* FP:context.rs-2679 */ }
/* FP:context.rs-2680 */ 
/* FP:context.rs-2681 */ impl<'tcx, T> Borrow<[T]> for InternedInSet<'tcx, ListWithCachedTypeInfo<T>> {
/* FP:context.rs-2682 */     fn borrow(&self) -> &[T] {
/* FP:context.rs-2683 */         &self.0[..]
/* FP:context.rs-2684 */     }
/* FP:context.rs-2685 */ }
/* FP:context.rs-2686 */ 
/* FP:context.rs-2687 */ impl<'tcx, T: PartialEq> PartialEq for InternedInSet<'tcx, ListWithCachedTypeInfo<T>> {
/* FP:context.rs-2688 */     fn eq(&self, other: &InternedInSet<'tcx, ListWithCachedTypeInfo<T>>) -> bool {
/* FP:context.rs-2689 */         // The `Borrow` trait requires that `x.borrow() == y.borrow()` equals
/* FP:context.rs-2690 */         // `x == y`.
/* FP:context.rs-2691 */         self.0[..] == other.0[..]
/* FP:context.rs-2692 */     }
/* FP:context.rs-2693 */ }
/* FP:context.rs-2694 */ 
/* FP:context.rs-2695 */ impl<'tcx, T: Eq> Eq for InternedInSet<'tcx, ListWithCachedTypeInfo<T>> {}
/* FP:context.rs-2696 */ 
/* FP:context.rs-2697 */ impl<'tcx, T: Hash> Hash for InternedInSet<'tcx, ListWithCachedTypeInfo<T>> {
/* FP:context.rs-2698 */     fn hash<H: Hasher>(&self, s: &mut H) {
/* FP:context.rs-2699 */         // The `Borrow` trait requires that `x.borrow().hash(s) == x.hash(s)`.
/* FP:context.rs-2700 */         self.0[..].hash(s)
/* FP:context.rs-2701 */     }
/* FP:context.rs-2702 */ }
/* FP:context.rs-2703 */ 
/* FP:context.rs-2704 */ macro_rules! direct_interners {
/* FP:context.rs-2705 */     ($($name:ident: $vis:vis $method:ident($ty:ty): $ret_ctor:ident -> $ret_ty:ty,)+) => {
/* FP:context.rs-2706 */         $(impl<'tcx> Borrow<$ty> for InternedInSet<'tcx, $ty> {
/* FP:context.rs-2707 */             fn borrow<'a>(&'a self) -> &'a $ty {
/* FP:context.rs-2708 */                 &self.0
/* FP:context.rs-2709 */             }
/* FP:context.rs-2710 */         }
/* FP:context.rs-2711 */ 
/* FP:context.rs-2712 */         impl<'tcx> PartialEq for InternedInSet<'tcx, $ty> {
/* FP:context.rs-2713 */             fn eq(&self, other: &Self) -> bool {
/* FP:context.rs-2714 */                 // The `Borrow` trait requires that `x.borrow() == y.borrow()`
/* FP:context.rs-2715 */                 // equals `x == y`.
/* FP:context.rs-2716 */                 self.0 == other.0
/* FP:context.rs-2717 */             }
/* FP:context.rs-2718 */         }
/* FP:context.rs-2719 */ 
/* FP:context.rs-2720 */         impl<'tcx> Eq for InternedInSet<'tcx, $ty> {}
/* FP:context.rs-2721 */ 
/* FP:context.rs-2722 */         impl<'tcx> Hash for InternedInSet<'tcx, $ty> {
/* FP:context.rs-2723 */             fn hash<H: Hasher>(&self, s: &mut H) {
/* FP:context.rs-2724 */                 // The `Borrow` trait requires that `x.borrow().hash(s) ==
/* FP:context.rs-2725 */                 // x.hash(s)`.
/* FP:context.rs-2726 */                 self.0.hash(s)
/* FP:context.rs-2727 */             }
/* FP:context.rs-2728 */         }
/* FP:context.rs-2729 */ 
/* FP:context.rs-2730 */         impl<'tcx> TyCtxt<'tcx> {
/* FP:context.rs-2731 */             $vis fn $method(self, v: $ty) -> $ret_ty {
/* FP:context.rs-2732 */                 $ret_ctor(Interned::new_unchecked(self.interners.$name.intern(v, |v| {
/* FP:context.rs-2733 */                     InternedInSet(self.interners.arena.alloc(v))
/* FP:context.rs-2734 */                 }).0))
/* FP:context.rs-2735 */             }
/* FP:context.rs-2736 */         })+
/* FP:context.rs-2737 */     }
/* FP:context.rs-2738 */ }
/* FP:context.rs-2739 */ 
/* FP:context.rs-2740 */ // Functions with a `mk_` prefix are intended for use outside this file and
/* FP:context.rs-2741 */ // crate. Functions with an `intern_` prefix are intended for use within this
/* FP:context.rs-2742 */ // crate only, and have a corresponding `mk_` function.
/* FP:context.rs-2743 */ direct_interners! {
/* FP:context.rs-2744 */     region: pub(crate) intern_region(RegionKind<'tcx>): Region -> Region<'tcx>,
/* FP:context.rs-2745 */     valtree: pub(crate) intern_valtree(ValTreeKind<'tcx>): ValTree -> ValTree<'tcx>,
/* FP:context.rs-2746 */     pat: pub mk_pat(PatternKind<'tcx>): Pattern -> Pattern<'tcx>,
/* FP:context.rs-2747 */     const_allocation: pub mk_const_alloc(Allocation): ConstAllocation -> ConstAllocation<'tcx>,
/* FP:context.rs-2748 */     layout: pub mk_layout(LayoutData<FieldIdx, VariantIdx>): Layout -> Layout<'tcx>,
/* FP:context.rs-2749 */     adt_def: pub mk_adt_def_from_data(AdtDefData): AdtDef -> AdtDef<'tcx>,
/* FP:context.rs-2750 */     external_constraints: pub mk_external_constraints(ExternalConstraintsData<TyCtxt<'tcx>>):
/* FP:context.rs-2751 */         ExternalConstraints -> ExternalConstraints<'tcx>,
/* FP:context.rs-2752 */     predefined_opaques_in_body: pub mk_predefined_opaques_in_body(PredefinedOpaquesData<TyCtxt<'tcx>>):
/* FP:context.rs-2753 */         PredefinedOpaques -> PredefinedOpaques<'tcx>,
/* FP:context.rs-2754 */ }
/* FP:context.rs-2755 */ 
/* FP:context.rs-2756 */ macro_rules! slice_interners {
/* FP:context.rs-2757 */     ($($field:ident: $vis:vis $method:ident($ty:ty)),+ $(,)?) => (
/* FP:context.rs-2758 */         impl<'tcx> TyCtxt<'tcx> {
/* FP:context.rs-2759 */             $($vis fn $method(self, v: &[$ty]) -> &'tcx List<$ty> {
/* FP:context.rs-2760 */                 if v.is_empty() {
/* FP:context.rs-2761 */                     List::empty()
/* FP:context.rs-2762 */                 } else {
/* FP:context.rs-2763 */                     self.interners.$field.intern_ref(v, || {
/* FP:context.rs-2764 */                         InternedInSet(List::from_arena(&*self.arena, (), v))
/* FP:context.rs-2765 */                     }).0
/* FP:context.rs-2766 */                 }
/* FP:context.rs-2767 */             })+
/* FP:context.rs-2768 */         }
/* FP:context.rs-2769 */     );
/* FP:context.rs-2770 */ }
/* FP:context.rs-2771 */ 
/* FP:context.rs-2772 */ // These functions intern slices. They all have a corresponding
/* FP:context.rs-2773 */ // `mk_foo_from_iter` function that interns an iterator. The slice version
/* FP:context.rs-2774 */ // should be used when possible, because it's faster.
/* FP:context.rs-2775 */ slice_interners!(
/* FP:context.rs-2776 */     const_lists: pub mk_const_list(Const<'tcx>),
/* FP:context.rs-2777 */     args: pub mk_args(GenericArg<'tcx>),
/* FP:context.rs-2778 */     type_lists: pub mk_type_list(Ty<'tcx>),
/* FP:context.rs-2779 */     canonical_var_kinds: pub mk_canonical_var_kinds(CanonicalVarKind<'tcx>),
/* FP:context.rs-2780 */     poly_existential_predicates: intern_poly_existential_predicates(PolyExistentialPredicate<'tcx>),
/* FP:context.rs-2781 */     projs: pub mk_projs(ProjectionKind),
/* FP:context.rs-2782 */     place_elems: pub mk_place_elems(PlaceElem<'tcx>),
/* FP:context.rs-2783 */     bound_variable_kinds: pub mk_bound_variable_kinds(ty::BoundVariableKind),
/* FP:context.rs-2784 */     fields: pub mk_fields(FieldIdx),
/* FP:context.rs-2785 */     local_def_ids: intern_local_def_ids(LocalDefId),
/* FP:context.rs-2786 */     captures: intern_captures(&'tcx ty::CapturedPlace<'tcx>),
/* FP:context.rs-2787 */     offset_of: pub mk_offset_of((VariantIdx, FieldIdx)),
/* FP:context.rs-2788 */     patterns: pub mk_patterns(Pattern<'tcx>),
/* FP:context.rs-2789 */     outlives: pub mk_outlives(ty::ArgOutlivesPredicate<'tcx>),
/* FP:context.rs-2790 */ );
/* FP:context.rs-2791 */ 
/* FP:context.rs-2792 */ impl<'tcx> TyCtxt<'tcx> {
/* FP:context.rs-2793 */     /// Given a `fn` type, returns an equivalent `unsafe fn` type;
/* FP:context.rs-2794 */     /// that is, a `fn` type that is equivalent in every way for being
/* FP:context.rs-2795 */     /// unsafe.
/* FP:context.rs-2796 */     pub fn safe_to_unsafe_fn_ty(self, sig: PolyFnSig<'tcx>) -> Ty<'tcx> {
/* FP:context.rs-2797 */         assert!(sig.safety().is_safe());
/* FP:context.rs-2798 */         Ty::new_fn_ptr(self, sig.map_bound(|sig| ty::FnSig { safety: hir::Safety::Unsafe, ..sig }))
/* FP:context.rs-2799 */     }
/* FP:context.rs-2800 */ 
/* FP:context.rs-2801 */     /// Given the def_id of a Trait `trait_def_id` and the name of an associated item `assoc_name`
/* FP:context.rs-2802 */     /// returns true if the `trait_def_id` defines an associated item of name `assoc_name`.
/* FP:context.rs-2803 */     pub fn trait_may_define_assoc_item(self, trait_def_id: DefId, assoc_name: Ident) -> bool {
/* FP:context.rs-2804 */         elaborate::supertrait_def_ids(self, trait_def_id).any(|trait_did| {
/* FP:context.rs-2805 */             self.associated_items(trait_did)
/* FP:context.rs-2806 */                 .filter_by_name_unhygienic(assoc_name.name)
/* FP:context.rs-2807 */                 .any(|item| self.hygienic_eq(assoc_name, item.ident(self), trait_did))
/* FP:context.rs-2808 */         })
/* FP:context.rs-2809 */     }
/* FP:context.rs-2810 */ 
/* FP:context.rs-2811 */     /// Given a `ty`, return whether it's an `impl Future<...>`.
/* FP:context.rs-2812 */     pub fn ty_is_opaque_future(self, ty: Ty<'_>) -> bool {
/* FP:context.rs-2813 */         let ty::Alias(ty::Opaque, ty::AliasTy { def_id, .. }) = ty.kind() else { return false };
/* FP:context.rs-2814 */         let future_trait = self.require_lang_item(LangItem::Future, DUMMY_SP);
/* FP:context.rs-2815 */ 
/* FP:context.rs-2816 */         self.explicit_item_self_bounds(def_id).skip_binder().iter().any(|&(predicate, _)| {
/* FP:context.rs-2817 */             let ty::ClauseKind::Trait(trait_predicate) = predicate.kind().skip_binder() else {
/* FP:context.rs-2818 */                 return false;
/* FP:context.rs-2819 */             };
/* FP:context.rs-2820 */             trait_predicate.trait_ref.def_id == future_trait
/* FP:context.rs-2821 */                 && trait_predicate.polarity == PredicatePolarity::Positive
/* FP:context.rs-2822 */         })
/* FP:context.rs-2823 */     }
/* FP:context.rs-2824 */ 
/* FP:context.rs-2825 */     /// Given a closure signature, returns an equivalent fn signature. Detuples
/* FP:context.rs-2826 */     /// and so forth -- so e.g., if we have a sig with `Fn<(u32, i32)>` then
/* FP:context.rs-2827 */     /// you would get a `fn(u32, i32)`.
/* FP:context.rs-2828 */     /// `unsafety` determines the unsafety of the fn signature. If you pass
/* FP:context.rs-2829 */     /// `hir::Safety::Unsafe` in the previous example, then you would get
/* FP:context.rs-2830 */     /// an `unsafe fn (u32, i32)`.
/* FP:context.rs-2831 */     /// It cannot convert a closure that requires unsafe.
/* FP:context.rs-2832 */     pub fn signature_unclosure(self, sig: PolyFnSig<'tcx>, safety: hir::Safety) -> PolyFnSig<'tcx> {
/* FP:context.rs-2833 */         sig.map_bound(|s| {
/* FP:context.rs-2834 */             let params = match s.inputs()[0].kind() {
/* FP:context.rs-2835 */                 ty::Tuple(params) => *params,
/* FP:context.rs-2836 */                 _ => bug!(),
/* FP:context.rs-2837 */             };
/* FP:context.rs-2838 */             self.mk_fn_sig(params, s.output(), s.c_variadic, safety, ExternAbi::Rust)
/* FP:context.rs-2839 */         })
/* FP:context.rs-2840 */     }
/* FP:context.rs-2841 */ 
/* FP:context.rs-2842 */     #[inline]
/* FP:context.rs-2843 */     pub fn mk_predicate(self, binder: Binder<'tcx, PredicateKind<'tcx>>) -> Predicate<'tcx> {
/* FP:context.rs-2844 */         self.interners.intern_predicate(
/* FP:context.rs-2845 */             binder,
/* FP:context.rs-2846 */             self.sess,
/* FP:context.rs-2847 */             // This is only used to create a stable hashing context.
/* FP:context.rs-2848 */             &self.untracked,
/* FP:context.rs-2849 */         )
/* FP:context.rs-2850 */     }
/* FP:context.rs-2851 */ 
/* FP:context.rs-2852 */     #[inline]
/* FP:context.rs-2853 */     pub fn reuse_or_mk_predicate(
/* FP:context.rs-2854 */         self,
/* FP:context.rs-2855 */         pred: Predicate<'tcx>,
/* FP:context.rs-2856 */         binder: Binder<'tcx, PredicateKind<'tcx>>,
/* FP:context.rs-2857 */     ) -> Predicate<'tcx> {
/* FP:context.rs-2858 */         if pred.kind() != binder { self.mk_predicate(binder) } else { pred }
/* FP:context.rs-2859 */     }
/* FP:context.rs-2860 */ 
/* FP:context.rs-2861 */     pub fn check_args_compatible(self, def_id: DefId, args: &'tcx [ty::GenericArg<'tcx>]) -> bool {
/* FP:context.rs-2862 */         self.check_args_compatible_inner(def_id, args, false)
/* FP:context.rs-2863 */     }
/* FP:context.rs-2864 */ 
/* FP:context.rs-2865 */     fn check_args_compatible_inner(
/* FP:context.rs-2866 */         self,
/* FP:context.rs-2867 */         def_id: DefId,
/* FP:context.rs-2868 */         args: &'tcx [ty::GenericArg<'tcx>],
/* FP:context.rs-2869 */         nested: bool,
/* FP:context.rs-2870 */     ) -> bool {
/* FP:context.rs-2871 */         let generics = self.generics_of(def_id);
/* FP:context.rs-2872 */ 
/* FP:context.rs-2873 */         // IATs themselves have a weird arg setup (self + own args), but nested items *in* IATs
/* FP:context.rs-2874 */         // (namely: opaques, i.e. ATPITs) do not.
/* FP:context.rs-2875 */         let own_args = if !nested
/* FP:context.rs-2876 */             && let DefKind::AssocTy = self.def_kind(def_id)
/* FP:context.rs-2877 */             && let DefKind::Impl { of_trait: false } = self.def_kind(self.parent(def_id))
/* FP:context.rs-2878 */         {
/* FP:context.rs-2879 */             if generics.own_params.len() + 1 != args.len() {
/* FP:context.rs-2880 */                 return false;
/* FP:context.rs-2881 */             }
/* FP:context.rs-2882 */ 
/* FP:context.rs-2883 */             if !matches!(args[0].kind(), ty::GenericArgKind::Type(_)) {
/* FP:context.rs-2884 */                 return false;
/* FP:context.rs-2885 */             }
/* FP:context.rs-2886 */ 
/* FP:context.rs-2887 */             &args[1..]
/* FP:context.rs-2888 */         } else {
/* FP:context.rs-2889 */             if generics.count() != args.len() {
/* FP:context.rs-2890 */                 return false;
/* FP:context.rs-2891 */             }
/* FP:context.rs-2892 */ 
/* FP:context.rs-2893 */             let (parent_args, own_args) = args.split_at(generics.parent_count);
/* FP:context.rs-2894 */ 
/* FP:context.rs-2895 */             if let Some(parent) = generics.parent
/* FP:context.rs-2896 */                 && !self.check_args_compatible_inner(parent, parent_args, true)
/* FP:context.rs-2897 */             {
/* FP:context.rs-2898 */                 return false;
/* FP:context.rs-2899 */             }
/* FP:context.rs-2900 */ 
/* FP:context.rs-2901 */             own_args
/* FP:context.rs-2902 */         };
/* FP:context.rs-2903 */ 
/* FP:context.rs-2904 */         for (param, arg) in std::iter::zip(&generics.own_params, own_args) {
/* FP:context.rs-2905 */             match (&param.kind, arg.kind()) {
/* FP:context.rs-2906 */                 (ty::GenericParamDefKind::Type { .. }, ty::GenericArgKind::Type(_))
/* FP:context.rs-2907 */                 | (ty::GenericParamDefKind::Lifetime, ty::GenericArgKind::Lifetime(_))
/* FP:context.rs-2908 */                 | (ty::GenericParamDefKind::Const { .. }, ty::GenericArgKind::Const(_)) => {}
/* FP:context.rs-2909 */                 _ => return false,
/* FP:context.rs-2910 */             }
/* FP:context.rs-2911 */         }
/* FP:context.rs-2912 */ 
/* FP:context.rs-2913 */         true
/* FP:context.rs-2914 */     }
/* FP:context.rs-2915 */ 
/* FP:context.rs-2916 */     /// With `cfg(debug_assertions)`, assert that args are compatible with their generics,
/* FP:context.rs-2917 */     /// and print out the args if not.
/* FP:context.rs-2918 */     pub fn debug_assert_args_compatible(self, def_id: DefId, args: &'tcx [ty::GenericArg<'tcx>]) {
/* FP:context.rs-2919 */         if cfg!(debug_assertions) && !self.check_args_compatible(def_id, args) {
/* FP:context.rs-2920 */             if let DefKind::AssocTy = self.def_kind(def_id)
/* FP:context.rs-2921 */                 && let DefKind::Impl { of_trait: false } = self.def_kind(self.parent(def_id))
/* FP:context.rs-2922 */             {
/* FP:context.rs-2923 */                 bug!(
/* FP:context.rs-2924 */                     "args not compatible with generics for {}: args={:#?}, generics={:#?}",
/* FP:context.rs-2925 */                     self.def_path_str(def_id),
/* FP:context.rs-2926 */                     args,
/* FP:context.rs-2927 */                     // Make `[Self, GAT_ARGS...]` (this could be simplified)
/* FP:context.rs-2928 */                     self.mk_args_from_iter(
/* FP:context.rs-2929 */                         [self.types.self_param.into()].into_iter().chain(
/* FP:context.rs-2930 */                             self.generics_of(def_id)
/* FP:context.rs-2931 */                                 .own_args(ty::GenericArgs::identity_for_item(self, def_id))
/* FP:context.rs-2932 */                                 .iter()
/* FP:context.rs-2933 */                                 .copied()
/* FP:context.rs-2934 */                         )
/* FP:context.rs-2935 */                     )
/* FP:context.rs-2936 */                 );
/* FP:context.rs-2937 */             } else {
/* FP:context.rs-2938 */                 bug!(
/* FP:context.rs-2939 */                     "args not compatible with generics for {}: args={:#?}, generics={:#?}",
/* FP:context.rs-2940 */                     self.def_path_str(def_id),
/* FP:context.rs-2941 */                     args,
/* FP:context.rs-2942 */                     ty::GenericArgs::identity_for_item(self, def_id)
/* FP:context.rs-2943 */                 );
/* FP:context.rs-2944 */             }
/* FP:context.rs-2945 */         }
/* FP:context.rs-2946 */     }
/* FP:context.rs-2947 */ 
/* FP:context.rs-2948 */     #[inline(always)]
/* FP:context.rs-2949 */     pub(crate) fn check_and_mk_args(
/* FP:context.rs-2950 */         self,
/* FP:context.rs-2951 */         def_id: DefId,
/* FP:context.rs-2952 */         args: impl IntoIterator<Item: Into<GenericArg<'tcx>>>,
/* FP:context.rs-2953 */     ) -> GenericArgsRef<'tcx> {
/* FP:context.rs-2954 */         let args = self.mk_args_from_iter(args.into_iter().map(Into::into));
/* FP:context.rs-2955 */         self.debug_assert_args_compatible(def_id, args);
/* FP:context.rs-2956 */         args
/* FP:context.rs-2957 */     }
/* FP:context.rs-2958 */ 
/* FP:context.rs-2959 */     #[inline]
/* FP:context.rs-2960 */     pub fn mk_ct_from_kind(self, kind: ty::ConstKind<'tcx>) -> Const<'tcx> {
/* FP:context.rs-2961 */         self.interners.intern_const(
/* FP:context.rs-2962 */             kind,
/* FP:context.rs-2963 */             self.sess,
/* FP:context.rs-2964 */             // This is only used to create a stable hashing context.
/* FP:context.rs-2965 */             &self.untracked,
/* FP:context.rs-2966 */         )
/* FP:context.rs-2967 */     }
/* FP:context.rs-2968 */ 
/* FP:context.rs-2969 */     // Avoid this in favour of more specific `Ty::new_*` methods, where possible.
/* FP:context.rs-2970 */     #[allow(rustc::usage_of_ty_tykind)]
/* FP:context.rs-2971 */     #[inline]
/* FP:context.rs-2972 */     pub fn mk_ty_from_kind(self, st: TyKind<'tcx>) -> Ty<'tcx> {
/* FP:context.rs-2973 */         self.interners.intern_ty(
/* FP:context.rs-2974 */             st,
/* FP:context.rs-2975 */             self.sess,
/* FP:context.rs-2976 */             // This is only used to create a stable hashing context.
/* FP:context.rs-2977 */             &self.untracked,
/* FP:context.rs-2978 */         )
/* FP:context.rs-2979 */     }
/* FP:context.rs-2980 */ 
/* FP:context.rs-2981 */     pub fn mk_param_from_def(self, param: &ty::GenericParamDef) -> GenericArg<'tcx> {
/* FP:context.rs-2982 */         match param.kind {
/* FP:context.rs-2983 */             GenericParamDefKind::Lifetime => {
/* FP:context.rs-2984 */                 ty::Region::new_early_param(self, param.to_early_bound_region_data()).into()
/* FP:context.rs-2985 */             }
/* FP:context.rs-2986 */             GenericParamDefKind::Type { .. } => Ty::new_param(self, param.index, param.name).into(),
/* FP:context.rs-2987 */             GenericParamDefKind::Const { .. } => {
/* FP:context.rs-2988 */                 ty::Const::new_param(self, ParamConst { index: param.index, name: param.name })
/* FP:context.rs-2989 */                     .into()
/* FP:context.rs-2990 */             }
/* FP:context.rs-2991 */         }
/* FP:context.rs-2992 */     }
/* FP:context.rs-2993 */ 
/* FP:context.rs-2994 */     pub fn mk_place_field(self, place: Place<'tcx>, f: FieldIdx, ty: Ty<'tcx>) -> Place<'tcx> {
/* FP:context.rs-2995 */         self.mk_place_elem(place, PlaceElem::Field(f, ty))
/* FP:context.rs-2996 */     }
/* FP:context.rs-2997 */ 
/* FP:context.rs-2998 */     pub fn mk_place_deref(self, place: Place<'tcx>) -> Place<'tcx> {
/* FP:context.rs-2999 */         self.mk_place_elem(place, PlaceElem::Deref)
/* FP:context.rs-3000 */     }
/* FP:context.rs-3001 */ 
/* FP:context.rs-3002 */     pub fn mk_place_downcast(
/* FP:context.rs-3003 */         self,
/* FP:context.rs-3004 */         place: Place<'tcx>,
/* FP:context.rs-3005 */         adt_def: AdtDef<'tcx>,
/* FP:context.rs-3006 */         variant_index: VariantIdx,
/* FP:context.rs-3007 */     ) -> Place<'tcx> {
/* FP:context.rs-3008 */         self.mk_place_elem(
/* FP:context.rs-3009 */             place,
/* FP:context.rs-3010 */             PlaceElem::Downcast(Some(adt_def.variant(variant_index).name), variant_index),
/* FP:context.rs-3011 */         )
/* FP:context.rs-3012 */     }
/* FP:context.rs-3013 */ 
/* FP:context.rs-3014 */     pub fn mk_place_downcast_unnamed(
/* FP:context.rs-3015 */         self,
/* FP:context.rs-3016 */         place: Place<'tcx>,
/* FP:context.rs-3017 */         variant_index: VariantIdx,
/* FP:context.rs-3018 */     ) -> Place<'tcx> {
/* FP:context.rs-3019 */         self.mk_place_elem(place, PlaceElem::Downcast(None, variant_index))
/* FP:context.rs-3020 */     }
/* FP:context.rs-3021 */ 
/* FP:context.rs-3022 */     pub fn mk_place_index(self, place: Place<'tcx>, index: Local) -> Place<'tcx> {
/* FP:context.rs-3023 */         self.mk_place_elem(place, PlaceElem::Index(index))
/* FP:context.rs-3024 */     }
/* FP:context.rs-3025 */ 
/* FP:context.rs-3026 */     /// This method copies `Place`'s projection, add an element and reintern it. Should not be used
/* FP:context.rs-3027 */     /// to build a full `Place` it's just a convenient way to grab a projection and modify it in
/* FP:context.rs-3028 */     /// flight.
/* FP:context.rs-3029 */     pub fn mk_place_elem(self, place: Place<'tcx>, elem: PlaceElem<'tcx>) -> Place<'tcx> {
/* FP:context.rs-3030 */         let mut projection = place.projection.to_vec();
/* FP:context.rs-3031 */         projection.push(elem);
/* FP:context.rs-3032 */ 
/* FP:context.rs-3033 */         Place { local: place.local, projection: self.mk_place_elems(&projection) }
/* FP:context.rs-3034 */     }
/* FP:context.rs-3035 */ 
/* FP:context.rs-3036 */     pub fn mk_poly_existential_predicates(
/* FP:context.rs-3037 */         self,
/* FP:context.rs-3038 */         eps: &[PolyExistentialPredicate<'tcx>],
/* FP:context.rs-3039 */     ) -> &'tcx List<PolyExistentialPredicate<'tcx>> {
/* FP:context.rs-3040 */         assert!(!eps.is_empty());
/* FP:context.rs-3041 */         assert!(
/* FP:context.rs-3042 */             eps.array_windows()
/* FP:context.rs-3043 */                 .all(|[a, b]| a.skip_binder().stable_cmp(self, &b.skip_binder())
/* FP:context.rs-3044 */                     != Ordering::Greater)
/* FP:context.rs-3045 */         );
/* FP:context.rs-3046 */         self.intern_poly_existential_predicates(eps)
/* FP:context.rs-3047 */     }
/* FP:context.rs-3048 */ 
/* FP:context.rs-3049 */     pub fn mk_clauses(self, clauses: &[Clause<'tcx>]) -> Clauses<'tcx> {
/* FP:context.rs-3050 */         // FIXME consider asking the input slice to be sorted to avoid
/* FP:context.rs-3051 */         // re-interning permutations, in which case that would be asserted
/* FP:context.rs-3052 */         // here.
/* FP:context.rs-3053 */         self.interners.intern_clauses(clauses)
/* FP:context.rs-3054 */     }
/* FP:context.rs-3055 */ 
/* FP:context.rs-3056 */     pub fn mk_local_def_ids(self, def_ids: &[LocalDefId]) -> &'tcx List<LocalDefId> {
/* FP:context.rs-3057 */         // FIXME consider asking the input slice to be sorted to avoid
/* FP:context.rs-3058 */         // re-interning permutations, in which case that would be asserted
/* FP:context.rs-3059 */         // here.
/* FP:context.rs-3060 */         self.intern_local_def_ids(def_ids)
/* FP:context.rs-3061 */     }
/* FP:context.rs-3062 */ 
/* FP:context.rs-3063 */     pub fn mk_patterns_from_iter<I, T>(self, iter: I) -> T::Output
/* FP:context.rs-3064 */     where
/* FP:context.rs-3065 */         I: Iterator<Item = T>,
/* FP:context.rs-3066 */         T: CollectAndApply<ty::Pattern<'tcx>, &'tcx List<ty::Pattern<'tcx>>>,
/* FP:context.rs-3067 */     {
/* FP:context.rs-3068 */         T::collect_and_apply(iter, |xs| self.mk_patterns(xs))
/* FP:context.rs-3069 */     }
/* FP:context.rs-3070 */ 
/* FP:context.rs-3071 */     pub fn mk_local_def_ids_from_iter<I, T>(self, iter: I) -> T::Output
/* FP:context.rs-3072 */     where
/* FP:context.rs-3073 */         I: Iterator<Item = T>,
/* FP:context.rs-3074 */         T: CollectAndApply<LocalDefId, &'tcx List<LocalDefId>>,
/* FP:context.rs-3075 */     {
/* FP:context.rs-3076 */         T::collect_and_apply(iter, |xs| self.mk_local_def_ids(xs))
/* FP:context.rs-3077 */     }
/* FP:context.rs-3078 */ 
/* FP:context.rs-3079 */     pub fn mk_captures_from_iter<I, T>(self, iter: I) -> T::Output
/* FP:context.rs-3080 */     where
/* FP:context.rs-3081 */         I: Iterator<Item = T>,
/* FP:context.rs-3082 */         T: CollectAndApply<
/* FP:context.rs-3083 */                 &'tcx ty::CapturedPlace<'tcx>,
/* FP:context.rs-3084 */                 &'tcx List<&'tcx ty::CapturedPlace<'tcx>>,
/* FP:context.rs-3085 */             >,
/* FP:context.rs-3086 */     {
/* FP:context.rs-3087 */         T::collect_and_apply(iter, |xs| self.intern_captures(xs))
/* FP:context.rs-3088 */     }
/* FP:context.rs-3089 */ 
/* FP:context.rs-3090 */     pub fn mk_const_list_from_iter<I, T>(self, iter: I) -> T::Output
/* FP:context.rs-3091 */     where
/* FP:context.rs-3092 */         I: Iterator<Item = T>,
/* FP:context.rs-3093 */         T: CollectAndApply<ty::Const<'tcx>, &'tcx List<ty::Const<'tcx>>>,
/* FP:context.rs-3094 */     {
/* FP:context.rs-3095 */         T::collect_and_apply(iter, |xs| self.mk_const_list(xs))
/* FP:context.rs-3096 */     }
/* FP:context.rs-3097 */ 
/* FP:context.rs-3098 */     // Unlike various other `mk_*_from_iter` functions, this one uses `I:
/* FP:context.rs-3099 */     // IntoIterator` instead of `I: Iterator`, and it doesn't have a slice
/* FP:context.rs-3100 */     // variant, because of the need to combine `inputs` and `output`. This
/* FP:context.rs-3101 */     // explains the lack of `_from_iter` suffix.
/* FP:context.rs-3102 */     pub fn mk_fn_sig<I, T>(
/* FP:context.rs-3103 */         self,
/* FP:context.rs-3104 */         inputs: I,
/* FP:context.rs-3105 */         output: I::Item,
/* FP:context.rs-3106 */         c_variadic: bool,
/* FP:context.rs-3107 */         safety: hir::Safety,
/* FP:context.rs-3108 */         abi: ExternAbi,
/* FP:context.rs-3109 */     ) -> T::Output
/* FP:context.rs-3110 */     where
/* FP:context.rs-3111 */         I: IntoIterator<Item = T>,
/* FP:context.rs-3112 */         T: CollectAndApply<Ty<'tcx>, ty::FnSig<'tcx>>,
/* FP:context.rs-3113 */     {
/* FP:context.rs-3114 */         T::collect_and_apply(inputs.into_iter().chain(iter::once(output)), |xs| ty::FnSig {
/* FP:context.rs-3115 */             inputs_and_output: self.mk_type_list(xs),
/* FP:context.rs-3116 */             c_variadic,
/* FP:context.rs-3117 */             safety,
/* FP:context.rs-3118 */             abi,
/* FP:context.rs-3119 */         })
/* FP:context.rs-3120 */     }
/* FP:context.rs-3121 */ 
/* FP:context.rs-3122 */     pub fn mk_poly_existential_predicates_from_iter<I, T>(self, iter: I) -> T::Output
/* FP:context.rs-3123 */     where
/* FP:context.rs-3124 */         I: Iterator<Item = T>,
/* FP:context.rs-3125 */         T: CollectAndApply<
/* FP:context.rs-3126 */                 PolyExistentialPredicate<'tcx>,
/* FP:context.rs-3127 */                 &'tcx List<PolyExistentialPredicate<'tcx>>,
/* FP:context.rs-3128 */             >,
/* FP:context.rs-3129 */     {
/* FP:context.rs-3130 */         T::collect_and_apply(iter, |xs| self.mk_poly_existential_predicates(xs))
/* FP:context.rs-3131 */     }
/* FP:context.rs-3132 */ 
/* FP:context.rs-3133 */     pub fn mk_clauses_from_iter<I, T>(self, iter: I) -> T::Output
/* FP:context.rs-3134 */     where
/* FP:context.rs-3135 */         I: Iterator<Item = T>,
/* FP:context.rs-3136 */         T: CollectAndApply<Clause<'tcx>, Clauses<'tcx>>,
/* FP:context.rs-3137 */     {
/* FP:context.rs-3138 */         T::collect_and_apply(iter, |xs| self.mk_clauses(xs))
/* FP:context.rs-3139 */     }
/* FP:context.rs-3140 */ 
/* FP:context.rs-3141 */     pub fn mk_type_list_from_iter<I, T>(self, iter: I) -> T::Output
/* FP:context.rs-3142 */     where
/* FP:context.rs-3143 */         I: Iterator<Item = T>,
/* FP:context.rs-3144 */         T: CollectAndApply<Ty<'tcx>, &'tcx List<Ty<'tcx>>>,
/* FP:context.rs-3145 */     {
/* FP:context.rs-3146 */         T::collect_and_apply(iter, |xs| self.mk_type_list(xs))
/* FP:context.rs-3147 */     }
/* FP:context.rs-3148 */ 
/* FP:context.rs-3149 */     pub fn mk_args_from_iter<I, T>(self, iter: I) -> T::Output
/* FP:context.rs-3150 */     where
/* FP:context.rs-3151 */         I: Iterator<Item = T>,
/* FP:context.rs-3152 */         T: CollectAndApply<GenericArg<'tcx>, ty::GenericArgsRef<'tcx>>,
/* FP:context.rs-3153 */     {
/* FP:context.rs-3154 */         T::collect_and_apply(iter, |xs| self.mk_args(xs))
/* FP:context.rs-3155 */     }
/* FP:context.rs-3156 */ 
/* FP:context.rs-3157 */     pub fn mk_canonical_var_infos_from_iter<I, T>(self, iter: I) -> T::Output
/* FP:context.rs-3158 */     where
/* FP:context.rs-3159 */         I: Iterator<Item = T>,
/* FP:context.rs-3160 */         T: CollectAndApply<CanonicalVarKind<'tcx>, &'tcx List<CanonicalVarKind<'tcx>>>,
/* FP:context.rs-3161 */     {
/* FP:context.rs-3162 */         T::collect_and_apply(iter, |xs| self.mk_canonical_var_kinds(xs))
/* FP:context.rs-3163 */     }
/* FP:context.rs-3164 */ 
/* FP:context.rs-3165 */     pub fn mk_place_elems_from_iter<I, T>(self, iter: I) -> T::Output
/* FP:context.rs-3166 */     where
/* FP:context.rs-3167 */         I: Iterator<Item = T>,
/* FP:context.rs-3168 */         T: CollectAndApply<PlaceElem<'tcx>, &'tcx List<PlaceElem<'tcx>>>,
/* FP:context.rs-3169 */     {
/* FP:context.rs-3170 */         T::collect_and_apply(iter, |xs| self.mk_place_elems(xs))
/* FP:context.rs-3171 */     }
/* FP:context.rs-3172 */ 
/* FP:context.rs-3173 */     pub fn mk_fields_from_iter<I, T>(self, iter: I) -> T::Output
/* FP:context.rs-3174 */     where
/* FP:context.rs-3175 */         I: Iterator<Item = T>,
/* FP:context.rs-3176 */         T: CollectAndApply<FieldIdx, &'tcx List<FieldIdx>>,
/* FP:context.rs-3177 */     {
/* FP:context.rs-3178 */         T::collect_and_apply(iter, |xs| self.mk_fields(xs))
/* FP:context.rs-3179 */     }
/* FP:context.rs-3180 */ 
/* FP:context.rs-3181 */     pub fn mk_offset_of_from_iter<I, T>(self, iter: I) -> T::Output
/* FP:context.rs-3182 */     where
/* FP:context.rs-3183 */         I: Iterator<Item = T>,
/* FP:context.rs-3184 */         T: CollectAndApply<(VariantIdx, FieldIdx), &'tcx List<(VariantIdx, FieldIdx)>>,
/* FP:context.rs-3185 */     {
/* FP:context.rs-3186 */         T::collect_and_apply(iter, |xs| self.mk_offset_of(xs))
/* FP:context.rs-3187 */     }
/* FP:context.rs-3188 */ 
/* FP:context.rs-3189 */     pub fn mk_args_trait(
/* FP:context.rs-3190 */         self,
/* FP:context.rs-3191 */         self_ty: Ty<'tcx>,
/* FP:context.rs-3192 */         rest: impl IntoIterator<Item = GenericArg<'tcx>>,
/* FP:context.rs-3193 */     ) -> GenericArgsRef<'tcx> {
/* FP:context.rs-3194 */         self.mk_args_from_iter(iter::once(self_ty.into()).chain(rest))
/* FP:context.rs-3195 */     }
/* FP:context.rs-3196 */ 
/* FP:context.rs-3197 */     pub fn mk_bound_variable_kinds_from_iter<I, T>(self, iter: I) -> T::Output
/* FP:context.rs-3198 */     where
/* FP:context.rs-3199 */         I: Iterator<Item = T>,
/* FP:context.rs-3200 */         T: CollectAndApply<ty::BoundVariableKind, &'tcx List<ty::BoundVariableKind>>,
/* FP:context.rs-3201 */     {
/* FP:context.rs-3202 */         T::collect_and_apply(iter, |xs| self.mk_bound_variable_kinds(xs))
/* FP:context.rs-3203 */     }
/* FP:context.rs-3204 */ 
/* FP:context.rs-3205 */     pub fn mk_outlives_from_iter<I, T>(self, iter: I) -> T::Output
/* FP:context.rs-3206 */     where
/* FP:context.rs-3207 */         I: Iterator<Item = T>,
/* FP:context.rs-3208 */         T: CollectAndApply<
/* FP:context.rs-3209 */                 ty::ArgOutlivesPredicate<'tcx>,
/* FP:context.rs-3210 */                 &'tcx ty::List<ty::ArgOutlivesPredicate<'tcx>>,
/* FP:context.rs-3211 */             >,
/* FP:context.rs-3212 */     {
/* FP:context.rs-3213 */         T::collect_and_apply(iter, |xs| self.mk_outlives(xs))
/* FP:context.rs-3214 */     }
/* FP:context.rs-3215 */ 
/* FP:context.rs-3216 */     /// Emit a lint at `span` from a lint struct (some type that implements `LintDiagnostic`,
/* FP:context.rs-3217 */     /// typically generated by `#[derive(LintDiagnostic)]`).
/* FP:context.rs-3218 */     #[track_caller]
/* FP:context.rs-3219 */     pub fn emit_node_span_lint(
/* FP:context.rs-3220 */         self,
/* FP:context.rs-3221 */         lint: &'static Lint,
/* FP:context.rs-3222 */         hir_id: HirId,
/* FP:context.rs-3223 */         span: impl Into<MultiSpan>,
/* FP:context.rs-3224 */         decorator: impl for<'a> LintDiagnostic<'a, ()>,
/* FP:context.rs-3225 */     ) {
/* FP:context.rs-3226 */         let level = self.lint_level_at_node(lint, hir_id);
/* FP:context.rs-3227 */         lint_level(self.sess, lint, level, Some(span.into()), |lint| {
/* FP:context.rs-3228 */             decorator.decorate_lint(lint);
/* FP:context.rs-3229 */         })
/* FP:context.rs-3230 */     }
/* FP:context.rs-3231 */ 
/* FP:context.rs-3232 */     /// Emit a lint at the appropriate level for a hir node, with an associated span.
/* FP:context.rs-3233 */     ///
/* FP:context.rs-3234 */     /// [`lint_level`]: crate::rustc_middle::lint::lint_level#decorate-signature
/* FP:context.rs-3235 */     #[rustc_lint_diagnostics]
/* FP:context.rs-3236 */     #[track_caller]
/* FP:context.rs-3237 */     pub fn node_span_lint(
/* FP:context.rs-3238 */         self,
/* FP:context.rs-3239 */         lint: &'static Lint,
/* FP:context.rs-3240 */         hir_id: HirId,
/* FP:context.rs-3241 */         span: impl Into<MultiSpan>,
/* FP:context.rs-3242 */         decorate: impl for<'a, 'b> FnOnce(&'b mut Diag<'a, ()>),
/* FP:context.rs-3243 */     ) {
/* FP:context.rs-3244 */         let level = self.lint_level_at_node(lint, hir_id);
/* FP:context.rs-3245 */         lint_level(self.sess, lint, level, Some(span.into()), decorate);
/* FP:context.rs-3246 */     }
/* FP:context.rs-3247 */ 
/* FP:context.rs-3248 */     /// Find the appropriate span where `use` and outer attributes can be inserted at.
/* FP:context.rs-3249 */     pub fn crate_level_attribute_injection_span(self) -> Span {
/* FP:context.rs-3250 */         let node = self.hir_node(hir::CRATE_HIR_ID);
/* FP:context.rs-3251 */         let hir::Node::Crate(m) = node else { bug!() };
/* FP:context.rs-3252 */         m.spans.inject_use_span.shrink_to_lo()
/* FP:context.rs-3253 */     }
/* FP:context.rs-3254 */ 
/* FP:context.rs-3255 */     pub fn disabled_nightly_features<E: crate::rustc_errors::EmissionGuarantee>(
/* FP:context.rs-3256 */         self,
/* FP:context.rs-3257 */         diag: &mut Diag<'_, E>,
/* FP:context.rs-3258 */         features: impl IntoIterator<Item = (String, Symbol)>,
/* FP:context.rs-3259 */     ) {
/* FP:context.rs-3260 */         if !self.sess.is_nightly_build() {
/* FP:context.rs-3261 */             return;
/* FP:context.rs-3262 */         }
/* FP:context.rs-3263 */ 
/* FP:context.rs-3264 */         let span = self.crate_level_attribute_injection_span();
/* FP:context.rs-3265 */         for (desc, feature) in features {
/* FP:context.rs-3266 */             // FIXME: make this string translatable
/* FP:context.rs-3267 */             let msg =
/* FP:context.rs-3268 */                 format!("add `#[feature({feature})]` to the crate attributes to enable{desc}");
/* FP:context.rs-3269 */             diag.span_suggestion_verbose(
/* FP:context.rs-3270 */                 span,
/* FP:context.rs-3271 */                 msg,
/* FP:context.rs-3272 */                 format!("#[feature({feature})]\n"),
/* FP:context.rs-3273 */                 Applicability::MaybeIncorrect,
/* FP:context.rs-3274 */             );
/* FP:context.rs-3275 */         }
/* FP:context.rs-3276 */     }
/* FP:context.rs-3277 */ 
/* FP:context.rs-3278 */     /// Emit a lint from a lint struct (some type that implements `LintDiagnostic`, typically
/* FP:context.rs-3279 */     /// generated by `#[derive(LintDiagnostic)]`).
/* FP:context.rs-3280 */     #[track_caller]
/* FP:context.rs-3281 */     pub fn emit_node_lint(
/* FP:context.rs-3282 */         self,
/* FP:context.rs-3283 */         lint: &'static Lint,
/* FP:context.rs-3284 */         id: HirId,
/* FP:context.rs-3285 */         decorator: impl for<'a> LintDiagnostic<'a, ()>,
/* FP:context.rs-3286 */     ) {
/* FP:context.rs-3287 */         self.node_lint(lint, id, |lint| {
/* FP:context.rs-3288 */             decorator.decorate_lint(lint);
/* FP:context.rs-3289 */         })
/* FP:context.rs-3290 */     }
/* FP:context.rs-3291 */ 
/* FP:context.rs-3292 */     /// Emit a lint at the appropriate level for a hir node.
/* FP:context.rs-3293 */     ///
/* FP:context.rs-3294 */     /// [`lint_level`]: crate::rustc_middle::lint::lint_level#decorate-signature
/* FP:context.rs-3295 */     #[rustc_lint_diagnostics]
/* FP:context.rs-3296 */     #[track_caller]
/* FP:context.rs-3297 */     pub fn node_lint(
/* FP:context.rs-3298 */         self,
/* FP:context.rs-3299 */         lint: &'static Lint,
/* FP:context.rs-3300 */         id: HirId,
/* FP:context.rs-3301 */         decorate: impl for<'a, 'b> FnOnce(&'b mut Diag<'a, ()>),
/* FP:context.rs-3302 */     ) {
/* FP:context.rs-3303 */         let level = self.lint_level_at_node(lint, id);
/* FP:context.rs-3304 */         lint_level(self.sess, lint, level, None, decorate);
/* FP:context.rs-3305 */     }
/* FP:context.rs-3306 */ 
/* FP:context.rs-3307 */     pub fn in_scope_traits(self, id: HirId) -> Option<&'tcx [TraitCandidate]> {
/* FP:context.rs-3308 */         let map = self.in_scope_traits_map(id.owner)?;
/* FP:context.rs-3309 */         let candidates = map.get(&id.local_id)?;
/* FP:context.rs-3310 */         Some(candidates)
/* FP:context.rs-3311 */     }
/* FP:context.rs-3312 */ 
/* FP:context.rs-3313 */     pub fn named_bound_var(self, id: HirId) -> Option<resolve_bound_vars::ResolvedArg> {
/* FP:context.rs-3314 */         debug!(?id, "named_region");
/* FP:context.rs-3315 */         self.named_variable_map(id.owner).get(&id.local_id).cloned()
/* FP:context.rs-3316 */     }
/* FP:context.rs-3317 */ 
/* FP:context.rs-3318 */     pub fn is_late_bound(self, id: HirId) -> bool {
/* FP:context.rs-3319 */         self.is_late_bound_map(id.owner).is_some_and(|set| set.contains(&id.local_id))
/* FP:context.rs-3320 */     }
/* FP:context.rs-3321 */ 
/* FP:context.rs-3322 */     pub fn late_bound_vars(self, id: HirId) -> &'tcx List<ty::BoundVariableKind> {
/* FP:context.rs-3323 */         self.mk_bound_variable_kinds(
/* FP:context.rs-3324 */             &self
/* FP:context.rs-3325 */                 .late_bound_vars_map(id.owner)
/* FP:context.rs-3326 */                 .get(&id.local_id)
/* FP:context.rs-3327 */                 .cloned()
/* FP:context.rs-3328 */                 .unwrap_or_else(|| bug!("No bound vars found for {}", self.hir_id_to_string(id))),
/* FP:context.rs-3329 */         )
/* FP:context.rs-3330 */     }
/* FP:context.rs-3331 */ 
/* FP:context.rs-3332 */     /// Given the def-id of an early-bound lifetime on an opaque corresponding to
/* FP:context.rs-3333 */     /// a duplicated captured lifetime, map it back to the early- or late-bound
/* FP:context.rs-3334 */     /// lifetime of the function from which it originally as captured. If it is
/* FP:context.rs-3335 */     /// a late-bound lifetime, this will represent the liberated (`ReLateParam`) lifetime
/* FP:context.rs-3336 */     /// of the signature.
/* FP:context.rs-3337 */     // FIXME(RPITIT): if we ever synthesize new lifetimes for RPITITs and not just
/* FP:context.rs-3338 */     // re-use the generics of the opaque, this function will need to be tweaked slightly.
/* FP:context.rs-3339 */     pub fn map_opaque_lifetime_to_parent_lifetime(
/* FP:context.rs-3340 */         self,
/* FP:context.rs-3341 */         mut opaque_lifetime_param_def_id: LocalDefId,
/* FP:context.rs-3342 */     ) -> ty::Region<'tcx> {
/* FP:context.rs-3343 */         debug_assert!(
/* FP:context.rs-3344 */             matches!(self.def_kind(opaque_lifetime_param_def_id), DefKind::LifetimeParam),
/* FP:context.rs-3345 */             "{opaque_lifetime_param_def_id:?} is a {}",
/* FP:context.rs-3346 */             self.def_descr(opaque_lifetime_param_def_id.to_def_id())
/* FP:context.rs-3347 */         );
/* FP:context.rs-3348 */ 
/* FP:context.rs-3349 */         loop {
/* FP:context.rs-3350 */             let parent = self.local_parent(opaque_lifetime_param_def_id);
/* FP:context.rs-3351 */             let lifetime_mapping = self.opaque_captured_lifetimes(parent);
/* FP:context.rs-3352 */ 
/* FP:context.rs-3353 */             let Some((lifetime, _)) = lifetime_mapping
/* FP:context.rs-3354 */                 .iter()
/* FP:context.rs-3355 */                 .find(|(_, duplicated_param)| *duplicated_param == opaque_lifetime_param_def_id)
/* FP:context.rs-3356 */             else {
/* FP:context.rs-3357 */                 bug!("duplicated lifetime param should be present");
/* FP:context.rs-3358 */             };
/* FP:context.rs-3359 */ 
/* FP:context.rs-3360 */             match *lifetime {
/* FP:context.rs-3361 */                 resolve_bound_vars::ResolvedArg::EarlyBound(ebv) => {
/* FP:context.rs-3362 */                     let new_parent = self.local_parent(ebv);
/* FP:context.rs-3363 */ 
/* FP:context.rs-3364 */                     // If we map to another opaque, then it should be a parent
/* FP:context.rs-3365 */                     // of the opaque we mapped from. Continue mapping.
/* FP:context.rs-3366 */                     if matches!(self.def_kind(new_parent), DefKind::OpaqueTy) {
/* FP:context.rs-3367 */                         debug_assert_eq!(self.local_parent(parent), new_parent);
/* FP:context.rs-3368 */                         opaque_lifetime_param_def_id = ebv;
/* FP:context.rs-3369 */                         continue;
/* FP:context.rs-3370 */                     }
/* FP:context.rs-3371 */ 
/* FP:context.rs-3372 */                     let generics = self.generics_of(new_parent);
/* FP:context.rs-3373 */                     return ty::Region::new_early_param(
/* FP:context.rs-3374 */                         self,
/* FP:context.rs-3375 */                         ty::EarlyParamRegion {
/* FP:context.rs-3376 */                             index: generics
/* FP:context.rs-3377 */                                 .param_def_id_to_index(self, ebv.to_def_id())
/* FP:context.rs-3378 */                                 .expect("early-bound var should be present in fn generics"),
/* FP:context.rs-3379 */                             name: self.item_name(ebv.to_def_id()),
/* FP:context.rs-3380 */                         },
/* FP:context.rs-3381 */                     );
/* FP:context.rs-3382 */                 }
/* FP:context.rs-3383 */                 resolve_bound_vars::ResolvedArg::LateBound(_, _, lbv) => {
/* FP:context.rs-3384 */                     let new_parent = self.local_parent(lbv);
/* FP:context.rs-3385 */                     return ty::Region::new_late_param(
/* FP:context.rs-3386 */                         self,
/* FP:context.rs-3387 */                         new_parent.to_def_id(),
/* FP:context.rs-3388 */                         ty::LateParamRegionKind::Named(lbv.to_def_id()),
/* FP:context.rs-3389 */                     );
/* FP:context.rs-3390 */                 }
/* FP:context.rs-3391 */                 resolve_bound_vars::ResolvedArg::Error(guar) => {
/* FP:context.rs-3392 */                     return ty::Region::new_error(self, guar);
/* FP:context.rs-3393 */                 }
/* FP:context.rs-3394 */                 _ => {
/* FP:context.rs-3395 */                     return ty::Region::new_error_with_message(
/* FP:context.rs-3396 */                         self,
/* FP:context.rs-3397 */                         self.def_span(opaque_lifetime_param_def_id),
/* FP:context.rs-3398 */                         "cannot resolve lifetime",
/* FP:context.rs-3399 */                     );
/* FP:context.rs-3400 */                 }
/* FP:context.rs-3401 */             }
/* FP:context.rs-3402 */         }
/* FP:context.rs-3403 */     }
/* FP:context.rs-3404 */ 
/* FP:context.rs-3405 */     /// Whether `def_id` is a stable const fn (i.e., doesn't need any feature gates to be called).
/* FP:context.rs-3406 */     ///
/* FP:context.rs-3407 */     /// When this is `false`, the function may still be callable as a `const fn` due to features
/* FP:context.rs-3408 */     /// being enabled!
/* FP:context.rs-3409 */     pub fn is_stable_const_fn(self, def_id: DefId) -> bool {
/* FP:context.rs-3410 */         self.is_const_fn(def_id)
/* FP:context.rs-3411 */             && match self.lookup_const_stability(def_id) {
/* FP:context.rs-3412 */                 None => true, // a fn in a non-staged_api crate
/* FP:context.rs-3413 */                 Some(stability) if stability.is_const_stable() => true,
/* FP:context.rs-3414 */                 _ => false,
/* FP:context.rs-3415 */             }
/* FP:context.rs-3416 */     }
/* FP:context.rs-3417 */ 
/* FP:context.rs-3418 */     /// Whether the trait impl is marked const. This does not consider stability or feature gates.
/* FP:context.rs-3419 */     pub fn is_const_trait_impl(self, def_id: DefId) -> bool {
/* FP:context.rs-3420 */         self.def_kind(def_id) == DefKind::Impl { of_trait: true }
/* FP:context.rs-3421 */             && self.impl_trait_header(def_id).unwrap().constness == hir::Constness::Const
/* FP:context.rs-3422 */     }
/* FP:context.rs-3423 */ 
/* FP:context.rs-3424 */     pub fn is_sdylib_interface_build(self) -> bool {
/* FP:context.rs-3425 */         self.sess.opts.unstable_opts.build_sdylib_interface
/* FP:context.rs-3426 */     }
/* FP:context.rs-3427 */ 
/* FP:context.rs-3428 */     pub fn intrinsic(self, def_id: impl IntoQueryParam<DefId> + Copy) -> Option<ty::IntrinsicDef> {
/* FP:context.rs-3429 */         match self.def_kind(def_id) {
/* FP:context.rs-3430 */             DefKind::Fn | DefKind::AssocFn => {}
/* FP:context.rs-3431 */             _ => return None,
/* FP:context.rs-3432 */         }
/* FP:context.rs-3433 */         self.intrinsic_raw(def_id)
/* FP:context.rs-3434 */     }
/* FP:context.rs-3435 */ 
/* FP:context.rs-3436 */     pub fn next_trait_solver_globally(self) -> bool {
/* FP:context.rs-3437 */         self.sess.opts.unstable_opts.next_solver.globally
/* FP:context.rs-3438 */     }
/* FP:context.rs-3439 */ 
/* FP:context.rs-3440 */     pub fn next_trait_solver_in_coherence(self) -> bool {
/* FP:context.rs-3441 */         self.sess.opts.unstable_opts.next_solver.coherence
/* FP:context.rs-3442 */     }
/* FP:context.rs-3443 */ 
/* FP:context.rs-3444 */     #[allow(rustc::bad_opt_access)]
/* FP:context.rs-3445 */     pub fn use_typing_mode_borrowck(self) -> bool {
/* FP:context.rs-3446 */         self.next_trait_solver_globally() || self.sess.opts.unstable_opts.typing_mode_borrowck
/* FP:context.rs-3447 */     }
/* FP:context.rs-3448 */ 
/* FP:context.rs-3449 */     pub fn is_impl_trait_in_trait(self, def_id: DefId) -> bool {
/* FP:context.rs-3450 */         self.opt_rpitit_info(def_id).is_some()
/* FP:context.rs-3451 */     }
/* FP:context.rs-3452 */ 
/* FP:context.rs-3453 */     /// Named module children from all kinds of items, including imports.
/* FP:context.rs-3454 */     /// In addition to regular items this list also includes struct and variant constructors, and
/* FP:context.rs-3455 */     /// items inside `extern {}` blocks because all of them introduce names into parent module.
/* FP:context.rs-3456 */     ///
/* FP:context.rs-3457 */     /// Module here is understood in name resolution sense - it can be a `mod` item,
/* FP:context.rs-3458 */     /// or a crate root, or an enum, or a trait.
/* FP:context.rs-3459 */     ///
/* FP:context.rs-3460 */     /// This is not a query, making it a query causes perf regressions
/* FP:context.rs-3461 */     /// (probably due to hashing spans in `ModChild`ren).
/* FP:context.rs-3462 */     pub fn module_children_local(self, def_id: LocalDefId) -> &'tcx [ModChild] {
/* FP:context.rs-3463 */         self.resolutions(()).module_children.get(&def_id).map_or(&[], |v| &v[..])
/* FP:context.rs-3464 */     }
/* FP:context.rs-3465 */ 
/* FP:context.rs-3466 */     /// Return the crate imported by given use item.
/* FP:context.rs-3467 */     pub fn extern_mod_stmt_cnum(self, def_id: LocalDefId) -> Option<CrateNum> {
/* FP:context.rs-3468 */         self.resolutions(()).extern_crate_map.get(&def_id).copied()
/* FP:context.rs-3469 */     }
/* FP:context.rs-3470 */ 
/* FP:context.rs-3471 */     pub fn resolver_for_lowering(self) -> &'tcx Steal<(ty::ResolverAstLowering, Arc<ast::Crate>)> {
/* FP:context.rs-3472 */         self.resolver_for_lowering_raw(()).0
/* FP:context.rs-3473 */     }
/* FP:context.rs-3474 */ 
/* FP:context.rs-3475 */     pub fn metadata_dep_node(self) -> crate::dep_graph::DepNode {
/* FP:context.rs-3476 */         crate::dep_graph::make_metadata(self)
/* FP:context.rs-3477 */     }
/* FP:context.rs-3478 */ 
/* FP:context.rs-3479 */     /// Given an `impl_id`, return the trait it implements.
/* FP:context.rs-3480 */     /// Return `None` if this is an inherent impl.
/* FP:context.rs-3481 */     pub fn impl_trait_ref(
/* FP:context.rs-3482 */         self,
/* FP:context.rs-3483 */         def_id: impl IntoQueryParam<DefId>,
/* FP:context.rs-3484 */     ) -> Option<ty::EarlyBinder<'tcx, ty::TraitRef<'tcx>>> {
/* FP:context.rs-3485 */         Some(self.impl_trait_header(def_id)?.trait_ref)
/* FP:context.rs-3486 */     }
/* FP:context.rs-3487 */ 
/* FP:context.rs-3488 */     pub fn impl_polarity(self, def_id: impl IntoQueryParam<DefId>) -> ty::ImplPolarity {
/* FP:context.rs-3489 */         self.impl_trait_header(def_id).map_or(ty::ImplPolarity::Positive, |h| h.polarity)
/* FP:context.rs-3490 */     }
/* FP:context.rs-3491 */ 
/* FP:context.rs-3492 */     pub fn needs_coroutine_by_move_body_def_id(self, def_id: DefId) -> bool {
/* FP:context.rs-3493 */         if let Some(hir::CoroutineKind::Desugared(_, hir::CoroutineSource::Closure)) =
/* FP:context.rs-3494 */             self.coroutine_kind(def_id)
/* FP:context.rs-3495 */             && let ty::Coroutine(_, args) = self.type_of(def_id).instantiate_identity().kind()
/* FP:context.rs-3496 */             && args.as_coroutine().kind_ty().to_opt_closure_kind() != Some(ty::ClosureKind::FnOnce)
/* FP:context.rs-3497 */         {
/* FP:context.rs-3498 */             true
/* FP:context.rs-3499 */         } else {
/* FP:context.rs-3500 */             false
/* FP:context.rs-3501 */         }
/* FP:context.rs-3502 */     }
/* FP:context.rs-3503 */ 
/* FP:context.rs-3504 */     /// Whether this is a trait implementation that has `#[diagnostic::do_not_recommend]`
/* FP:context.rs-3505 */     pub fn do_not_recommend_impl(self, def_id: DefId) -> bool {
/* FP:context.rs-3506 */         self.get_diagnostic_attr(def_id, sym::do_not_recommend).is_some()
/* FP:context.rs-3507 */     }
/* FP:context.rs-3508 */ 
/* FP:context.rs-3509 */     /// Whether this def is one of the special bin crate entrypoint functions that must have a
/* FP:context.rs-3510 */     /// monomorphization and also not be internalized in the bin crate.
/* FP:context.rs-3511 */     pub fn is_entrypoint(self, def_id: DefId) -> bool {
/* FP:context.rs-3512 */         if self.is_lang_item(def_id, LangItem::Start) {
/* FP:context.rs-3513 */             return true;
/* FP:context.rs-3514 */         }
/* FP:context.rs-3515 */         if let Some((entry_def_id, _)) = self.entry_fn(())
/* FP:context.rs-3516 */             && entry_def_id == def_id
/* FP:context.rs-3517 */         {
/* FP:context.rs-3518 */             return true;
/* FP:context.rs-3519 */         }
/* FP:context.rs-3520 */         false
/* FP:context.rs-3521 */     }
/* FP:context.rs-3522 */ }
/* FP:context.rs-3523 */ 
/* FP:context.rs-3524 */ /// Parameter attributes that can only be determined by examining the body of a function instead
/* FP:context.rs-3525 */ /// of just its signature.
/* FP:context.rs-3526 */ ///
/* FP:context.rs-3527 */ /// These can be useful for optimization purposes when a function is directly called. We compute
/* FP:context.rs-3528 */ /// them and store them into the crate metadata so that downstream crates can make use of them.
/* FP:context.rs-3529 */ ///
/* FP:context.rs-3530 */ /// Right now, we only have `read_only`, but `no_capture` and `no_alias` might be useful in the
/* FP:context.rs-3531 */ /// future.
/* FP:context.rs-3532 */ #[derive(Clone, Copy, PartialEq, Debug, Default, TyDecodable, TyEncodable, HashStable)]
/* FP:context.rs-3533 */ pub struct DeducedParamAttrs {
/* FP:context.rs-3534 */     /// The parameter is marked immutable in the function and contains no `UnsafeCell` (i.e. its
/* FP:context.rs-3535 */     /// type is freeze).
/* FP:context.rs-3536 */     pub read_only: bool,
/* FP:context.rs-3537 */ }
/* FP:context.rs-3538 */ 
/* FP:context.rs-3539 */ pub fn provide(providers: &mut Providers) {
/* FP:context.rs-3540 */     providers.is_panic_runtime =
/* FP:context.rs-3541 */         |tcx, LocalCrate| contains_name(tcx.hir_krate_attrs(), sym::panic_runtime);
/* FP:context.rs-3542 */     providers.is_compiler_builtins =
/* FP:context.rs-3543 */         |tcx, LocalCrate| contains_name(tcx.hir_krate_attrs(), sym::compiler_builtins);
/* FP:context.rs-3544 */     providers.has_panic_handler = |tcx, LocalCrate| {
/* FP:context.rs-3545 */         // We want to check if the panic handler was defined in this crate
/* FP:context.rs-3546 */         tcx.lang_items().panic_impl().is_some_and(|did| did.is_local())
/* FP:context.rs-3547 */     };
/* FP:context.rs-3548 */     providers.source_span = |tcx, def_id| tcx.untracked.source_span.get(def_id).unwrap_or(DUMMY_SP);
/* FP:context.rs-3549 */ }
/* FP:context.rs-3550 */ 
/* FP:context.rs-3551 */ pub fn contains_name(attrs: &[Attribute], name: Symbol) -> bool {
/* FP:context.rs-3552 */     attrs.iter().any(|x| x.has_name(name))
/* FP:context.rs-3553 */ }