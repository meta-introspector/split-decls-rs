// Generated macro for test_checked_sub_offset (function)
macro_rules! Depcrate_naive_datetime_teststest_checked_sub_offset {
() => {
// Module: crate::naive::datetime::tests
// Provides: {"test_checked_sub_offset"}
// Dependencies: {}
# [test] fn test_checked_sub_offset () { let ymdhmsm = | y , m , d , h , mn , s , mi | { NaiveDate :: from_ymd_opt (y , m , d) . unwrap () . and_hms_milli_opt (h , mn , s , mi) } ; let positive_offset = FixedOffset :: east_opt (2 * 60 * 60) . unwrap () ; let dt = ymdhmsm (2023 , 5 , 5 , 20 , 10 , 0 , 0) . unwrap () ; assert_eq ! (dt . checked_sub_offset (positive_offset) , ymdhmsm (2023 , 5 , 5 , 18 , 10 , 0 , 0)) ; let dt = ymdhmsm (2023 , 6 , 30 , 23 , 59 , 59 , 1_000) . unwrap () ; assert_eq ! (dt . checked_sub_offset (positive_offset) , ymdhmsm (2023 , 6 , 30 , 21 , 59 , 59 , 1_000)) ; assert ! (NaiveDateTime :: MIN . checked_sub_offset (positive_offset) . is_none ()) ; let negative_offset = FixedOffset :: west_opt (2 * 60 * 60) . unwrap () ; let dt = ymdhmsm (2023 , 5 , 5 , 20 , 10 , 0 , 0) . unwrap () ; assert_eq ! (dt . checked_sub_offset (negative_offset) , ymdhmsm (2023 , 5 , 5 , 22 , 10 , 0 , 0)) ; let dt = ymdhmsm (2023 , 6 , 30 , 23 , 59 , 59 , 1_000) . unwrap () ; assert_eq ! (dt . checked_sub_offset (negative_offset) , ymdhmsm (2023 , 7 , 1 , 1 , 59 , 59 , 1_000)) ; assert ! (NaiveDateTime :: MAX . checked_sub_offset (negative_offset) . is_none ()) ; assert_eq ! (dt . checked_add_offset (positive_offset) , Some (dt + positive_offset)) ; assert_eq ! (dt . checked_sub_offset (positive_offset) , Some (dt - positive_offset)) ; }
};
}
