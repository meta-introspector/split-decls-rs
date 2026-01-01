/* FP:query.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_USE_0001
/* FP:query.rs-0002 */ use rustc_macros :: { HashStable , TypeFoldable , TypeVisitable } ;
/* FP:query.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_USE_0002
/* FP:query.rs-0004 */ use crate :: rustc_complete :: Span ;
/* FP:query.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_USE_0003
/* FP:query.rs-0006 */ use crate :: error :: DropCheckOverflow ;
/* FP:query.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_USE_0004
/* FP:query.rs-0008 */ use crate :: infer :: canonical :: { Canonical , CanonicalQueryInput , QueryResponse } ;
/* FP:query.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_USE_0005
/* FP:query.rs-0010 */ pub use crate :: traits :: solve :: NoSolution ;
/* FP:query.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_USE_0006
/* FP:query.rs-0012 */ use crate :: ty :: { self , GenericArg , Ty , TyCtxt } ;
/* FP:query.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_MOD_0007
/* FP:query.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_TYPE_0008
/* FP:query.rs-0016 */ pub type CanonicalAliasGoal < 'tcx > = CanonicalQueryInput < 'tcx , ty :: ParamEnvAnd < 'tcx , ty :: AliasTy < 'tcx > > > ;
/* FP:query.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_TYPE_0009
/* FP:query.rs-0018 */ pub type CanonicalTyGoal < 'tcx > = CanonicalQueryInput < 'tcx , ty :: ParamEnvAnd < 'tcx , Ty < 'tcx > > > ;
/* FP:query.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_TYPE_0010
/* FP:query.rs-0020 */ pub type CanonicalPredicateGoal < 'tcx > = CanonicalQueryInput < 'tcx , ty :: ParamEnvAnd < 'tcx , ty :: Predicate < 'tcx > > > ;
/* FP:query.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_TYPE_0011
/* FP:query.rs-0022 */ pub type CanonicalTypeOpAscribeUserTypeGoal < 'tcx > = CanonicalQueryInput < 'tcx , ty :: ParamEnvAnd < 'tcx , type_op :: AscribeUserType < 'tcx > > > ;
/* FP:query.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_TYPE_0012
/* FP:query.rs-0024 */ pub type CanonicalTypeOpProvePredicateGoal < 'tcx > = CanonicalQueryInput < 'tcx , ty :: ParamEnvAnd < 'tcx , type_op :: ProvePredicate < 'tcx > > > ;
/* FP:query.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_TYPE_0013
/* FP:query.rs-0026 */ pub type CanonicalTypeOpNormalizeGoal < 'tcx , T > = CanonicalQueryInput < 'tcx , ty :: ParamEnvAnd < 'tcx , type_op :: Normalize < T > > > ;
/* FP:query.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_TYPE_0014
/* FP:query.rs-0028 */ pub type CanonicalTypeOpDeeplyNormalizeGoal < 'tcx , T > = CanonicalQueryInput < 'tcx , ty :: ParamEnvAnd < 'tcx , type_op :: DeeplyNormalize < T > > > ;
/* FP:query.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_TYPE_0015
/* FP:query.rs-0030 */ pub type CanonicalImpliedOutlivesBoundsGoal < 'tcx > = CanonicalQueryInput < 'tcx , ty :: ParamEnvAnd < 'tcx , type_op :: ImpliedOutlivesBounds < 'tcx > > > ;
/* FP:query.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_TYPE_0016
/* FP:query.rs-0032 */ pub type CanonicalDropckOutlivesGoal < 'tcx > = CanonicalQueryInput < 'tcx , ty :: ParamEnvAnd < 'tcx , type_op :: DropckOutlives < 'tcx > > > ;
/* FP:query.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_STRUCT_0017
/* FP:query.rs-0034 */ # [derive (Clone , Debug , Default , HashStable , TypeFoldable , TypeVisitable)] pub struct DropckOutlivesResult < 'tcx > { pub kinds : Vec < GenericArg < 'tcx > > , pub overflows : Vec < Ty < 'tcx > > , }
/* FP:query.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_IMPL_0018
/* FP:query.rs-0036 */ impl < 'tcx > DropckOutlivesResult < 'tcx > { pub fn report_overflows (& self , tcx : TyCtxt < 'tcx > , span : Span , ty : Ty < 'tcx >) { if let Some (overflow_ty) = self . overflows . get (0) { tcx . dcx () . emit_err (DropCheckOverflow { span , ty , overflow_ty : * overflow_ty }) ; } } }
/* FP:query.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_STRUCT_0019
/* FP:query.rs-0038 */ # [doc = " A set of constraints that need to be satisfied in order for"] # [doc = " a type to be valid for destruction."] # [derive (Clone , Debug , HashStable)] pub struct DropckConstraint < 'tcx > { # [doc = " Types that are required to be alive in order for this"] # [doc = " type to be valid for destruction."] pub outlives : Vec < ty :: GenericArg < 'tcx > > , # [doc = " Types that could not be resolved: projections and params."] pub dtorck_types : Vec < Ty < 'tcx > > , # [doc = " If, during the computation of the dtorck constraint, we"] # [doc = " overflow, that gets recorded here. The caller is expected to"] # [doc = " report an error."] pub overflows : Vec < Ty < 'tcx > > , }
/* FP:query.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_IMPL_0020
/* FP:query.rs-0040 */ impl < 'tcx > DropckConstraint < 'tcx > { pub fn empty () -> DropckConstraint < 'tcx > { DropckConstraint { outlives : vec ! [] , dtorck_types : vec ! [] , overflows : vec ! [] } } }
/* FP:query.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_IMPL_0021
/* FP:query.rs-0042 */ impl < 'tcx > FromIterator < DropckConstraint < 'tcx > > for DropckConstraint < 'tcx > { fn from_iter < I : IntoIterator < Item = DropckConstraint < 'tcx > > > (iter : I) -> Self { let mut result = Self :: empty () ; for DropckConstraint { outlives , dtorck_types , overflows } in iter { result . outlives . extend (outlives) ; result . dtorck_types . extend (dtorck_types) ; result . overflows . extend (overflows) ; } result } }
/* FP:query.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_STRUCT_0022
/* FP:query.rs-0044 */ # [derive (Debug , HashStable)] pub struct CandidateStep < 'tcx > { pub self_ty : Canonical < 'tcx , QueryResponse < 'tcx , Ty < 'tcx > > > , pub autoderefs : usize , # [doc = " `true` if the type results from a dereference of a raw pointer."] # [doc = " when assembling candidates, we include these steps, but not when"] # [doc = " picking methods. This so that if we have `foo: *const Foo` and `Foo` has methods"] # [doc = " `fn by_raw_ptr(self: *const Self)` and `fn by_ref(&self)`, then"] # [doc = " `foo.by_raw_ptr()` will work and `foo.by_ref()` won't."] pub from_unsafe_deref : bool , pub unsize : bool , # [doc = " We will generate CandidateSteps which are reachable via a chain"] # [doc = " of following `Receiver`. The first 'n' of those will be reachable"] # [doc = " by following a chain of 'Deref' instead (since there's a blanket"] # [doc = " implementation of Receiver for Deref)."] # [doc = " We use the entire set of steps when identifying method candidates"] # [doc = " (e.g. identifying relevant `impl` blocks) but only those that are"] # [doc = " reachable via Deref when examining what the receiver type can"] # [doc = " be converted into by autodereffing."] pub reachable_via_deref : bool , }
/* FP:query.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_STRUCT_0023
/* FP:query.rs-0046 */ # [derive (Copy , Clone , Debug , HashStable)] pub struct MethodAutoderefStepsResult < 'tcx > { # [doc = " The valid autoderef steps that could be found by following a chain"] # [doc = " of `Receiver<Target=T>` or `Deref<Target=T>` trait implementations."] pub steps : & 'tcx [CandidateStep < 'tcx >] , # [doc = " If Some(T), a type autoderef reported an error on."] pub opt_bad_ty : Option < & 'tcx MethodAutoderefBadTy < 'tcx > > , # [doc = " If `true`, `steps` has been truncated due to reaching the"] # [doc = " recursion limit."] pub reached_recursion_limit : bool , }
/* FP:query.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_STRUCT_0024
/* FP:query.rs-0048 */ # [derive (Debug , HashStable)] pub struct MethodAutoderefBadTy < 'tcx > { pub reached_raw_pointer : bool , pub ty : Canonical < 'tcx , QueryResponse < 'tcx , Ty < 'tcx > > > , }
/* FP:query.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_STRUCT_0025
/* FP:query.rs-0050 */ # [doc = " Result of the `normalize_canonicalized_{{,inherent_}projection,free}_ty` queries."] # [derive (Clone , Debug , HashStable , TypeFoldable , TypeVisitable)] pub struct NormalizationResult < 'tcx > { # [doc = " Result of the normalization."] pub normalized_ty : Ty < 'tcx > , }
/* FP:query.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_query_ENUM_0026
/* FP:query.rs-0052 */ # [doc = " Outlives bounds are relationships between generic parameters,"] # [doc = " whether they both be regions (`'a: 'b`) or whether types are"] # [doc = " involved (`T: 'a`). These relationships can be extracted from the"] # [doc = " full set of predicates we understand or also from types (in which"] # [doc = " case they are called implied bounds). They are fed to the"] # [doc = " `OutlivesEnv` which in turn is supplied to the region checker and"] # [doc = " other parts of the inference system."] # [derive (Copy , Clone , Debug , TypeFoldable , TypeVisitable , HashStable)] pub enum OutlivesBound < 'tcx > { RegionSubRegion (ty :: Region < 'tcx > , ty :: Region < 'tcx >) , RegionSubParam (ty :: Region < 'tcx > , ty :: ParamTy) , RegionSubAlias (ty :: Region < 'tcx > , ty :: AliasTy < 'tcx >) , }