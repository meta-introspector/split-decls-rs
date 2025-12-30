// Generated macro for month_add_overflowing (function)
macro_rules! Depcrate_civil_datemonth_add_overflowing {
() => {
// Module: crate::civil::date
// Provides: {"month_add_overflowing"}
// Dependencies: {}
# [doc = " Adds the given span of months to the `month` given."] # [doc = ""] # [doc = " If adding (or subtracting) would result in overflowing the `month` value,"] # [doc = " then the amount by which it overflowed, in units of years, is returned. For"] # [doc = " example, adding 14 months to the month `3` (March) will result in returning"] # [doc = " the month `5` (May) with `1` year of overflow."] fn month_add_overflowing (month : t :: Month , span : t :: SpanMonths ,) -> (t :: Month , t :: SpanYears) { let month = t :: SpanMonths :: rfrom (month) ; let total = month - C (1) + span ; let years = total / C (12) ; let month = (total % C (12)) + C (1) ; (month . rinto () , years . rinto ()) }
};
}
