// Generated macro for test_moment_to_rata_die_conversion (function)
macro_rules! Depcrate_rata_dietest_moment_to_rata_die_conversion {
() => {
// Module: crate::rata_die
// Provides: {"test_moment_to_rata_die_conversion"}
// Dependencies: {}
# [test] fn test_moment_to_rata_die_conversion () { for i in - 1000 ..= 1000 { let moment = Moment :: new (i as f64) ; let rata_die = moment . as_rata_die () ; assert_eq ! (rata_die . to_i64_date () , i) ; } }
};
}
