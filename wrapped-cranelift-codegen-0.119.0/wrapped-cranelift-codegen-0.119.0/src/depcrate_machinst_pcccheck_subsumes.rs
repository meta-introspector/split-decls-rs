// Generated macro for check_subsumes (function)
macro_rules! Depcrate_machinst_pcccheck_subsumes {
() => {
// Module: crate::machinst::pcc
// Provides: {"check_subsumes"}
// Dependencies: {}
pub (crate) fn check_subsumes (ctx : & FactContext , subsumer : & Fact , subsumee : & Fact) -> PccResult < () > { check_subsumes_optionals (ctx , Some (subsumer) , Some (subsumee)) }
};
}
