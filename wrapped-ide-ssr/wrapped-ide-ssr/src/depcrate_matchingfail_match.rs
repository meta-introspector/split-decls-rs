// Generated macro for fail_match (macro)
macro_rules! Depcrate_matchingfail_match {
() => {
// Module: crate::matching
// Provides: {"fail_match"}
// Dependencies: {}
macro_rules ! fail_match { ($ ($ args : tt) *) => { return Err (match_error ! ($ ($ args) *)) } ; }
};
}
