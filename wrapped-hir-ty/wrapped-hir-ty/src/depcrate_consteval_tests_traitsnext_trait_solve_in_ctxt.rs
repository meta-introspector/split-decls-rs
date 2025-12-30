// Generated macro for next_trait_solve_in_ctxt (function)
macro_rules! Depcrate_consteval_tests_traitsnext_trait_solve_in_ctxt {
() => {
// Module: crate::consteval::tests::traits
// Provides: {"next_trait_solve_in_ctxt"}
// Dependencies: {}
# [doc = " Solve a trait goal using next trait solver."] pub fn next_trait_solve_in_ctxt < 'db , 'a > (infer_ctxt : & 'a InferCtxt < 'db > , goal : Goal < 'db , Predicate < 'db > > ,) -> Result < (HasChanged , Certainty) , rustc_type_ir :: solve :: NoSolution > { tracing :: info ! (? goal) ; let context = < & SolverContext < 'db > > :: from (infer_ctxt) ; let res = context . evaluate_root_goal (goal , Span :: dummy () , None) ; let res = res . map (| r | (r . has_changed , r . certainty)) ; tracing :: debug ! ("solve_nextsolver({:?}) => {:?}" , goal , res) ; res }
};
}
