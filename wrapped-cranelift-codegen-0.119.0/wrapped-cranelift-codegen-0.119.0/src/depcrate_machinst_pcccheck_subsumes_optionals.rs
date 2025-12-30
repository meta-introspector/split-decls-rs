// Generated macro for check_subsumes_optionals (function)
macro_rules! Depcrate_machinst_pcccheck_subsumes_optionals {
() => {
// Module: crate::machinst::pcc
// Provides: {"check_subsumes_optionals"}
// Dependencies: {}
pub (crate) fn check_subsumes_optionals (ctx : & FactContext , subsumer : Option < & Fact > , subsumee : Option < & Fact > ,) -> PccResult < () > { trace ! ("checking if derived fact {:?} subsumes stated fact {:?}" , subsumer , subsumee) ; if ctx . subsumes_fact_optionals (subsumer , subsumee) { Ok (()) } else { Err (PccError :: UnsupportedFact) } }
};
}
