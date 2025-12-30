// Generated macro for tests (module)
macro_rules! Depcrate_persiantests {
() => {
// Module: crate::persian
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_persian_epoch () { let epoch = FIXED_PERSIAN_EPOCH . to_i64_date () ; let epoch_year_from_fixed = crate :: gregorian :: year_from_fixed (RataDie :: new (epoch)) . unwrap () ; assert_eq ! (epoch_year_from_fixed , 622) ; } fn nowruz (g_year : i32) -> RataDie { let (y , _m , _d) = crate :: gregorian :: gregorian_from_fixed (FIXED_PERSIAN_EPOCH) . unwrap () ; let persian_year = g_year - y + 1 ; let year = if persian_year <= 0 { persian_year - 1 } else { persian_year } ; fixed_from_fast_persian (year , 1 , 1) } # [test] fn test_nowruz () { let nowruz_test_year_start = 2000 ; let nowruz_test_year_end = 2103 ; for year in nowruz_test_year_start ..= nowruz_test_year_end { let two_thousand_eight_to_fixed = nowruz (year) . to_i64_date () ; let gregorian_date = crate :: gregorian :: fixed_from_gregorian (year , 3 , 21) ; let (persian_year , _m , _d) = fast_persian_from_fixed (gregorian_date) . unwrap () ; assert_eq ! (fast_persian_from_fixed (RataDie :: new (two_thousand_eight_to_fixed)) . unwrap () . 0 , persian_year) ; } } }
};
}
