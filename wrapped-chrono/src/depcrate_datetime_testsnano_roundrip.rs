// Generated macro for nano_roundrip (function)
macro_rules! Depcrate_datetime_testsnano_roundrip {
() => {
// Module: crate::datetime::tests
// Provides: {"nano_roundrip"}
// Dependencies: {}
# [doc = " This is an extended test for <https://github.com/chronotope/chrono/issues/1289>."] # [test] fn nano_roundrip () { const BILLION : i64 = 1_000_000_000 ; for nanos in [i64 :: MIN , i64 :: MIN + 1 , i64 :: MIN + 2 , i64 :: MIN + BILLION - 1 , i64 :: MIN + BILLION , i64 :: MIN + BILLION + 1 , - BILLION - 1 , - BILLION , - BILLION + 1 , 0 , BILLION - 1 , BILLION , BILLION + 1 , i64 :: MAX - BILLION - 1 , i64 :: MAX - BILLION , i64 :: MAX - BILLION + 1 , i64 :: MAX - 2 , i64 :: MAX - 1 , i64 :: MAX ,] { println ! ("nanos: {nanos}") ; let dt = Utc . timestamp_nanos (nanos) ; let nanos2 = dt . timestamp_nanos_opt () . expect ("value roundtrips") ; assert_eq ! (nanos , nanos2) ; } }
};
}
