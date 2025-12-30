// Generated macro for test_time_sub (function)
macro_rules! Depcrate_naive_time_teststest_time_sub {
() => {
// Module: crate::naive::time::tests
// Provides: {"test_time_sub"}
// Dependencies: {}
# [test] fn test_time_sub () { macro_rules ! check { ($ lhs : expr , $ rhs : expr , $ diff : expr) => { { assert_eq ! ($ lhs . signed_duration_since ($ rhs) , $ diff) ; assert_eq ! ($ rhs . signed_duration_since ($ lhs) , -$ diff) ; } } ; } let hmsm = | h , m , s , ms | NaiveTime :: from_hms_milli_opt (h , m , s , ms) . unwrap () ; check ! (hmsm (3 , 5 , 7 , 900) , hmsm (3 , 5 , 7 , 900) , TimeDelta :: zero ()) ; check ! (hmsm (3 , 5 , 7 , 900) , hmsm (3 , 5 , 7 , 600) , TimeDelta :: try_milliseconds (300) . unwrap ()) ; check ! (hmsm (3 , 5 , 7 , 200) , hmsm (2 , 4 , 6 , 200) , TimeDelta :: try_seconds (3600 + 60 + 1) . unwrap ()) ; check ! (hmsm (3 , 5 , 7 , 200) , hmsm (2 , 4 , 6 , 300) , TimeDelta :: try_seconds (3600 + 60) . unwrap () + TimeDelta :: try_milliseconds (900) . unwrap ()) ; check ! (hmsm (3 , 6 , 0 , 200) , hmsm (3 , 5 , 59 , 1_800) , TimeDelta :: try_milliseconds (400) . unwrap ()) ; assert_eq ! (hmsm (3 , 5 , 6 , 800) + TimeDelta :: try_milliseconds (400) . unwrap () , hmsm (3 , 5 , 7 , 200)) ; }
};
}
