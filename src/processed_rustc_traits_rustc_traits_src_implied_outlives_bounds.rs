/* FP:implied_outlives_bounds.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_implied_outlives_bounds_USE_0001
/* FP:implied_outlives_bounds.rs-0002 */ use crate :: rustc_infer :: infer :: TyCtxtInferExt ;
/* FP:implied_outlives_bounds.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_implied_outlives_bounds_USE_0002
/* FP:implied_outlives_bounds.rs-0004 */ use crate :: rustc_infer :: infer :: canonical :: { self , Canonical } ;
/* FP:implied_outlives_bounds.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_implied_outlives_bounds_USE_0003
/* FP:implied_outlives_bounds.rs-0006 */ use crate :: rustc_infer :: traits :: query :: OutlivesBound ;
/* FP:implied_outlives_bounds.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_implied_outlives_bounds_USE_0004
/* FP:implied_outlives_bounds.rs-0008 */ use crate :: rustc_infer :: traits :: query :: type_op :: ImpliedOutlivesBounds ;
/* FP:implied_outlives_bounds.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_implied_outlives_bounds_USE_0005
/* FP:implied_outlives_bounds.rs-0010 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:implied_outlives_bounds.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_implied_outlives_bounds_USE_0006
/* FP:implied_outlives_bounds.rs-0012 */ use crate :: rustc_complete :: ty :: { ParamEnvAnd , TyCtxt } ;
/* FP:implied_outlives_bounds.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_implied_outlives_bounds_USE_0007
/* FP:implied_outlives_bounds.rs-0014 */ use crate :: rustc_complete :: DUMMY_SP ;
/* FP:implied_outlives_bounds.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_implied_outlives_bounds_USE_0008
/* FP:implied_outlives_bounds.rs-0016 */ use crate :: rustc_trait_selection :: infer :: InferCtxtBuilderExt ;
/* FP:implied_outlives_bounds.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_implied_outlives_bounds_USE_0009
/* FP:implied_outlives_bounds.rs-0018 */ use crate :: rustc_trait_selection :: traits :: query :: type_op :: implied_outlives_bounds :: compute_implied_outlives_bounds_inner ;
/* FP:implied_outlives_bounds.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_implied_outlives_bounds_USE_0010
/* FP:implied_outlives_bounds.rs-0020 */ use crate :: rustc_trait_selection :: traits :: query :: { CanonicalImpliedOutlivesBoundsGoal , NoSolution } ;
/* FP:implied_outlives_bounds.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_implied_outlives_bounds_FN_0011
/* FP:implied_outlives_bounds.rs-0022 */ pub (crate) fn provide (p : & mut Providers) { * p = Providers { implied_outlives_bounds , .. * p } ; }
/* FP:implied_outlives_bounds.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_implied_outlives_bounds_FN_0012
/* FP:implied_outlives_bounds.rs-0024 */ fn implied_outlives_bounds < 'tcx > (tcx : TyCtxt < 'tcx > , (goal , disable_implied_bounds_hack) : (CanonicalImpliedOutlivesBoundsGoal < 'tcx > , bool) ,) -> Result < & 'tcx Canonical < 'tcx , canonical :: QueryResponse < 'tcx , Vec < OutlivesBound < 'tcx > > > > , NoSolution , > { tcx . infer_ctxt () . enter_canonical_trait_query (& goal , | ocx , key | { let ParamEnvAnd { param_env , value : ImpliedOutlivesBounds { ty } } = key ; compute_implied_outlives_bounds_inner (ocx , param_env , ty , DUMMY_SP , disable_implied_bounds_hack ,) }) }