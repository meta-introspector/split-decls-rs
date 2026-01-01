/* FP:solve.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_solve_USE_0001
/* FP:solve.rs-0002 */ pub use rustc_next_trait_solver :: solve :: * ;
/* FP:solve.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_solve_MOD_0002
/* FP:solve.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_solve_MOD_0003
/* FP:solve.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_solve_MOD_0004
/* FP:solve.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_solve_MOD_0005
/* FP:solve.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_solve_MOD_0006
/* FP:solve.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_solve_USE_0007
/* FP:solve.rs-0014 */ pub (crate) use delegate :: SolverDelegate ;
/* FP:solve.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_solve_USE_0008
/* FP:solve.rs-0016 */ pub use fulfill :: { FulfillmentCtxt , NextSolverError , StalledOnCoroutines } ;
/* FP:solve.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_solve_USE_0009
/* FP:solve.rs-0018 */ pub (crate) use normalize :: deeply_normalize_for_diagnostics ;
/* FP:solve.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_solve_USE_0010
/* FP:solve.rs-0020 */ pub use normalize :: { deeply_normalize , deeply_normalize_with_skipped_universes , deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals , } ;
/* FP:solve.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_solve_USE_0011
/* FP:solve.rs-0022 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:solve.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_solve_USE_0012
/* FP:solve.rs-0024 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:solve.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_solve_USE_0013
/* FP:solve.rs-0026 */ pub use select :: InferCtxtSelectExt ;
/* FP:solve.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_solve_FN_0014
/* FP:solve.rs-0028 */ fn evaluate_root_goal_for_proof_tree_raw < 'tcx > (tcx : TyCtxt < 'tcx > , canonical_input : CanonicalInput < TyCtxt < 'tcx > > ,) -> (QueryResult < TyCtxt < 'tcx > > , & 'tcx inspect :: Probe < TyCtxt < 'tcx > >) { evaluate_root_goal_for_proof_tree_raw_provider :: < SolverDelegate < 'tcx > , TyCtxt < 'tcx > > (tcx , canonical_input ,) }
/* FP:solve.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_solve_FN_0015
/* FP:solve.rs-0030 */ pub fn provide (providers : & mut Providers) { * providers = Providers { evaluate_root_goal_for_proof_tree_raw , .. * providers } ; }