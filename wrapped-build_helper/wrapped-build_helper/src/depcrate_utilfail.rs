// Generated macro for fail (function)
macro_rules! Depcrate_utilfail {
() => {
// Module: crate::util
// Provides: {"fail"}
// Dependencies: {}
pub fn fail (s : & str) -> ! { eprintln ! ("\n\n{s}\n\n") ; detail_exit (1 , cfg ! (test)) ; }
};
}
