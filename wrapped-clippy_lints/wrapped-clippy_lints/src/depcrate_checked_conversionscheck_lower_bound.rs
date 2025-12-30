// Generated macro for check_lower_bound (function)
macro_rules! Depcrate_checked_conversionscheck_lower_bound {
() => {
// Module: crate::checked_conversions
// Provides: {"check_lower_bound"}
// Dependencies: {}
# [doc = " Check for `expr >= 0|(to_type::MIN as from_type)`"] fn check_lower_bound < 'tcx > (lt : & 'tcx Expr < 'tcx > , gt : & 'tcx Expr < 'tcx >) -> Option < Conversion < 'tcx > > { check_lower_bound_zero (gt , lt) . or_else (| | check_lower_bound_min (gt , lt)) }
};
}
