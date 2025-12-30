// Generated macro for signed_duration_since_autoref (function)
macro_rules! Depcrate_datetime_testssigned_duration_since_autoref {
() => {
// Module: crate::datetime::tests
// Provides: {"signed_duration_since_autoref"}
// Dependencies: {}
# [test] # [allow (clippy :: needless_borrow , clippy :: op_ref)] fn signed_duration_since_autoref () { let dt1 = Utc . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () ; let dt2 = Utc . with_ymd_and_hms (2014 , 3 , 4 , 5 , 6 , 7) . unwrap () ; let diff1 = dt1 . signed_duration_since (dt2) ; # [allow (clippy :: needless_borrows_for_generic_args)] let diff2 = dt2 . signed_duration_since (& dt1) ; assert_eq ! (diff1 , - diff2) ; let diff1 = dt1 - & dt2 ; let diff2 = dt2 - dt1 ; assert_eq ! (diff1 , - diff2) ; }
};
}
