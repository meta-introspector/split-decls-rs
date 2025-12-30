// Generated macro for test_years_elapsed (function)
macro_rules! Depcrate_datetime_teststest_years_elapsed {
() => {
// Module: crate::datetime::tests
// Provides: {"test_years_elapsed"}
// Dependencies: {}
# [test] # [cfg (feature = "clock")] fn test_years_elapsed () { let one_year_ago = Utc :: now () . date_naive () - Days :: new (400) ; let two_year_ago = Utc :: now () . date_naive () - Days :: new (750) ; assert_eq ! (Utc :: now () . date_naive () . years_since (one_year_ago) , Some (1)) ; assert_eq ! (Utc :: now () . date_naive () . years_since (two_year_ago) , Some (2)) ; let future = Utc :: now () . date_naive () + Days (100) ; assert_eq ! (Utc :: now () . date_naive () . years_since (future) , None) ; }
};
}
