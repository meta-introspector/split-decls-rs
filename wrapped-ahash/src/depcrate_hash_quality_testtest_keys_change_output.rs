// Generated macro for test_keys_change_output (function)
macro_rules! Depcrate_hash_quality_testtest_keys_change_output {
() => {
// Module: crate::hash_quality_test
// Provides: {"test_keys_change_output"}
// Dependencies: {}
fn test_keys_change_output < T : Hasher > (constructor : impl Fn (u128 , u128) -> T) { let mut a = constructor (1 , 1) ; let mut b = constructor (1 , 2) ; let mut c = constructor (2 , 1) ; let mut d = constructor (2 , 2) ; "test" . hash (& mut a) ; "test" . hash (& mut b) ; "test" . hash (& mut c) ; "test" . hash (& mut d) ; assert_sufficiently_different (a . finish () , b . finish () , 1) ; assert_sufficiently_different (a . finish () , c . finish () , 1) ; assert_sufficiently_different (a . finish () , d . finish () , 1) ; assert_sufficiently_different (b . finish () , c . finish () , 1) ; assert_sufficiently_different (b . finish () , d . finish () , 1) ; assert_sufficiently_different (c . finish () , d . finish () , 1) ; }
};
}
