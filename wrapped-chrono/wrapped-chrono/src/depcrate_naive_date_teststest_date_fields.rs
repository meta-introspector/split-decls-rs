// Generated macro for test_date_fields (function)
macro_rules! Depcrate_naive_date_teststest_date_fields {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_date_fields"}
// Dependencies: {}
# [test] fn test_date_fields () { fn check (year : i32 , month : u32 , day : u32 , ordinal : u32) { let d1 = NaiveDate :: from_ymd_opt (year , month , day) . unwrap () ; assert_eq ! (d1 . year () , year) ; assert_eq ! (d1 . month () , month) ; assert_eq ! (d1 . day () , day) ; assert_eq ! (d1 . ordinal () , ordinal) ; let d2 = NaiveDate :: from_yo_opt (year , ordinal) . unwrap () ; assert_eq ! (d2 . year () , year) ; assert_eq ! (d2 . month () , month) ; assert_eq ! (d2 . day () , day) ; assert_eq ! (d2 . ordinal () , ordinal) ; assert_eq ! (d1 , d2) ; } check (2012 , 1 , 1 , 1) ; check (2012 , 1 , 2 , 2) ; check (2012 , 2 , 1 , 32) ; check (2012 , 2 , 29 , 60) ; check (2012 , 3 , 1 , 61) ; check (2012 , 4 , 9 , 100) ; check (2012 , 7 , 18 , 200) ; check (2012 , 10 , 26 , 300) ; check (2012 , 12 , 31 , 366) ; check (2014 , 1 , 1 , 1) ; check (2014 , 1 , 2 , 2) ; check (2014 , 2 , 1 , 32) ; check (2014 , 2 , 28 , 59) ; check (2014 , 3 , 1 , 60) ; check (2014 , 4 , 10 , 100) ; check (2014 , 7 , 19 , 200) ; check (2014 , 10 , 27 , 300) ; check (2014 , 12 , 31 , 365) ; }
};
}
