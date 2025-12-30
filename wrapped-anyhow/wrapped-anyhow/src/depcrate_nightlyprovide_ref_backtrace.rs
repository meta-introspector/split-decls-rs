// Generated macro for provide_ref_backtrace (function)
macro_rules! Depcrate_nightlyprovide_ref_backtrace {
() => {
// Module: crate::nightly
// Provides: {"provide_ref_backtrace"}
// Dependencies: {}
pub fn provide_ref_backtrace < 'a > (request : & mut Request < 'a > , backtrace : & 'a Backtrace) { Request :: provide_ref (request , backtrace) ; }
};
}
