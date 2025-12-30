// Generated macro for no_backtrace (function)
macro_rules! Depcrate_errorno_backtrace {
() => {
// Module: crate::error
// Provides: {"no_backtrace"}
// Dependencies: {}
# [cfg (all (not (error_generic_member_access) , any (std_backtrace , feature = "backtrace")))] fn no_backtrace (e : Ref < ErrorImpl >) -> Option < & Backtrace > { let _ = e ; None }
};
}
