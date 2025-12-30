// Generated macro for check_lower_bound_zero (function)
macro_rules! Depcrate_checked_conversionscheck_lower_bound_zero {
() => {
// Module: crate::checked_conversions
// Provides: {"check_lower_bound_zero"}
// Dependencies: {}
# [doc = " Check for `expr >= 0`"] fn check_lower_bound_zero < 'a > (candidate : & 'a Expr < '_ > , check : & 'a Expr < '_ >) -> Option < Conversion < 'a > > { is_integer_literal (check , 0) . then (| | Conversion :: new_any (candidate)) }
};
}
