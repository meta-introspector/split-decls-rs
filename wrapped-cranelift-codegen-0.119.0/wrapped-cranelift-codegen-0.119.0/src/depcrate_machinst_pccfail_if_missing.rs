// Generated macro for fail_if_missing (function)
macro_rules! Depcrate_machinst_pccfail_if_missing {
() => {
// Module: crate::machinst::pcc
// Provides: {"fail_if_missing"}
// Dependencies: {}
pub (crate) fn fail_if_missing (fact : Option < Fact >) -> PccResult < Fact > { fact . ok_or (PccError :: UnsupportedFact) }
};
}
