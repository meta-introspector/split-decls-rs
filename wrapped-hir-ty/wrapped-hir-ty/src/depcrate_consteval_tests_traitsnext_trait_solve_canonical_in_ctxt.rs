// Generated macro for next_trait_solve_canonical_in_ctxt (function)
macro_rules! Depcrate_consteval_tests_traitsnext_trait_solve_canonical_in_ctxt {
() => {
// Module: crate::consteval::tests::traits
// Provides: {"next_trait_solve_canonical_in_ctxt"}
// Dependencies: {}
pub fn next_trait_solve_canonical_in_ctxt < 'db > (infer_ctxt : & InferCtxt < 'db > , goal : Canonical < 'db , Goal < 'db , Predicate < 'db > > > ,) -> NextTraitSolveResult { infer_ctxt . probe (| _ | { let context = < & SolverContext < 'db > > :: from (infer_ctxt) ; tracing :: info ! (? goal) ; let (goal , var_values) = context . instantiate_canonical (& goal) ; tracing :: info ! (? var_values) ; let res = context . evaluate_root_goal (goal , Span :: dummy () , None) ; let res = res . map (| r | (r . has_changed , r . certainty)) ; tracing :: debug ! ("solve_nextsolver({:?}) => {:?}" , goal , res) ; match res { Err (_) => NextTraitSolveResult :: NoSolution , Ok ((_ , Certainty :: Yes)) => NextTraitSolveResult :: Certain , Ok ((_ , Certainty :: Maybe { .. })) => NextTraitSolveResult :: Uncertain , } }) }
};
}
