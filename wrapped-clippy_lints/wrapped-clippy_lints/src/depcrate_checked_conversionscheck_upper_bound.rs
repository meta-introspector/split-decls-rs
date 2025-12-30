// Generated macro for check_upper_bound (function)
macro_rules! Depcrate_checked_conversionscheck_upper_bound {
() => {
// Module: crate::checked_conversions
// Provides: {"check_upper_bound"}
// Dependencies: {}
# [doc = " Check for `expr <= (to_type::MAX as from_type)`"] fn check_upper_bound < 'tcx > (lt : & 'tcx Expr < 'tcx > , gt : & 'tcx Expr < 'tcx >) -> Option < Conversion < 'tcx > > { if let Some ((from , to)) = get_types_from_cast (gt , INTS , sym :: max_value , sym :: MAX) { Conversion :: try_new (lt , from , to) } else { None } }
};
}
