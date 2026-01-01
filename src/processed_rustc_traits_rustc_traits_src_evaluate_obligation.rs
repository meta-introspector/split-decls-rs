/* FP:evaluate_obligation.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_evaluate_obligation_USE_0001
/* FP:evaluate_obligation.rs-0002 */ use crate :: rustc_infer :: infer :: TyCtxtInferExt ;
/* FP:evaluate_obligation.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_evaluate_obligation_USE_0002
/* FP:evaluate_obligation.rs-0004 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:evaluate_obligation.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_evaluate_obligation_USE_0003
/* FP:evaluate_obligation.rs-0006 */ use crate :: rustc_complete :: ty :: { ParamEnvAnd , TyCtxt } ;
/* FP:evaluate_obligation.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_evaluate_obligation_USE_0004
/* FP:evaluate_obligation.rs-0008 */ use crate :: rustc_complete :: DUMMY_SP ;
/* FP:evaluate_obligation.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_evaluate_obligation_USE_0005
/* FP:evaluate_obligation.rs-0010 */ use crate :: rustc_trait_selection :: traits :: query :: CanonicalPredicateGoal ;
/* FP:evaluate_obligation.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_evaluate_obligation_USE_0006
/* FP:evaluate_obligation.rs-0012 */ use crate :: rustc_trait_selection :: traits :: { EvaluationResult , Obligation , ObligationCause , OverflowError , SelectionContext , TraitQueryMode , sizedness_fast_path , } ;
/* FP:evaluate_obligation.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_evaluate_obligation_USE_0007
/* FP:evaluate_obligation.rs-0014 */ use tracing :: debug ;
/* FP:evaluate_obligation.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_evaluate_obligation_FN_0008
/* FP:evaluate_obligation.rs-0016 */ pub (crate) fn provide (p : & mut Providers) { * p = Providers { evaluate_obligation , .. * p } ; }
/* FP:evaluate_obligation.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_traits_src_evaluate_obligation_FN_0009
/* FP:evaluate_obligation.rs-0018 */ fn evaluate_obligation < 'tcx > (tcx : TyCtxt < 'tcx > , canonical_goal : CanonicalPredicateGoal < 'tcx > ,) -> Result < EvaluationResult , OverflowError > { assert ! (! tcx . next_trait_solver_globally ()) ; debug ! ("evaluate_obligation(canonical_goal={:#?})" , canonical_goal) ; let (ref infcx , goal , _var_values) = tcx . infer_ctxt () . build_with_canonical (DUMMY_SP , & canonical_goal) ; debug ! ("evaluate_obligation: goal={:#?}" , goal) ; let ParamEnvAnd { param_env , value : predicate } = goal ; if sizedness_fast_path (tcx , predicate , param_env) { return Ok (EvaluationResult :: EvaluatedToOk) ; } let mut selcx = SelectionContext :: with_query_mode (infcx , TraitQueryMode :: Canonical) ; let obligation = Obligation :: new (tcx , ObligationCause :: dummy () , param_env , predicate) ; selcx . evaluate_root_obligation (& obligation) }