// Generated macro for test_finish_is_consistent (function)
macro_rules! Depcrate_hash_quality_testtest_finish_is_consistent {
() => {
// Module: crate::hash_quality_test
// Provides: {"test_finish_is_consistent"}
// Dependencies: {}
fn test_finish_is_consistent < T : Hasher > (constructor : impl Fn (u128 , u128) -> T) { let mut hasher = constructor (1 , 2) ; "Foo" . hash (& mut hasher) ; let a = hasher . finish () ; let b = hasher . finish () ; assert_eq ! (a , b) ; }
};
}
