// Generated macro for check_lower_bound_min (function)
macro_rules! Depcrate_checked_conversionscheck_lower_bound_min {
() => {
// Module: crate::checked_conversions
// Provides: {"check_lower_bound_min"}
// Dependencies: {}
# [doc = " Check for `expr >= (to_type::MIN as from_type)`"] fn check_lower_bound_min < 'a > (candidate : & 'a Expr < '_ > , check : & 'a Expr < '_ >) -> Option < Conversion < 'a > > { if let Some ((from , to)) = get_types_from_cast (check , SINTS , sym :: min_value , sym :: MIN) { Conversion :: try_new (candidate , from , to) } else { None } }
};
}
