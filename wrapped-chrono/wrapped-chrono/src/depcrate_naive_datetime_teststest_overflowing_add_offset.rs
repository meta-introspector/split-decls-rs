// Generated macro for test_overflowing_add_offset (function)
macro_rules! Depcrate_naive_datetime_teststest_overflowing_add_offset {
() => {
// Module: crate::naive::datetime::tests
// Provides: {"test_overflowing_add_offset"}
// Dependencies: {}
# [test] fn test_overflowing_add_offset () { let ymdhmsm = | y , m , d , h , mn , s , mi | { NaiveDate :: from_ymd_opt (y , m , d) . unwrap () . and_hms_milli_opt (h , mn , s , mi) . unwrap () } ; let positive_offset = FixedOffset :: east_opt (2 * 60 * 60) . unwrap () ; let dt = ymdhmsm (2023 , 5 , 5 , 20 , 10 , 0 , 0) ; assert_eq ! (dt . overflowing_add_offset (positive_offset) , ymdhmsm (2023 , 5 , 5 , 22 , 10 , 0 , 0)) ; let dt = ymdhmsm (2023 , 6 , 30 , 23 , 59 , 59 , 1_000) ; assert_eq ! (dt . overflowing_add_offset (positive_offset) , ymdhmsm (2023 , 7 , 1 , 1 , 59 , 59 , 1_000)) ; assert ! (NaiveDateTime :: MAX . overflowing_add_offset (positive_offset) > NaiveDateTime :: MAX) ; let negative_offset = FixedOffset :: west_opt (2 * 60 * 60) . unwrap () ; let dt = ymdhmsm (2023 , 5 , 5 , 20 , 10 , 0 , 0) ; assert_eq ! (dt . overflowing_add_offset (negative_offset) , ymdhmsm (2023 , 5 , 5 , 18 , 10 , 0 , 0)) ; let dt = ymdhmsm (2023 , 6 , 30 , 23 , 59 , 59 , 1_000) ; assert_eq ! (dt . overflowing_add_offset (negative_offset) , ymdhmsm (2023 , 6 , 30 , 21 , 59 , 59 , 1_000)) ; assert ! (NaiveDateTime :: MIN . overflowing_add_offset (negative_offset) < NaiveDateTime :: MIN) ; }
};
}
