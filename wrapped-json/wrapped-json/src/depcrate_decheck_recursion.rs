// Generated macro for check_recursion (macro)
macro_rules! Depcrate_decheck_recursion {
() => {
// Module: crate::de
// Provides: {"check_recursion"}
// Dependencies: {}
macro_rules ! check_recursion { ($ this : ident $ ($ body : tt) *) => { if_checking_recursion_limit ! { $ this . remaining_depth -= 1 ; if $ this . remaining_depth == 0 { return Err ($ this . peek_error (ErrorCode :: RecursionLimitExceeded)) ; } } $ this $ ($ body) * if_checking_recursion_limit ! { $ this . remaining_depth += 1 ; } } ; }
};
}
