// Generated macro for test_input_affect_every_byte (function)
macro_rules! Depcrate_hash_quality_testtest_input_affect_every_byte {
() => {
// Module: crate::hash_quality_test
// Provides: {"test_input_affect_every_byte"}
// Dependencies: {}
fn test_input_affect_every_byte < T : Hasher > (constructor : impl Fn (u128 , u128) -> T) { let base = hash_with (& 0 , constructor (0 , 0)) ; for shift in 0 .. 16 { let mut alternatives = vec ! [] ; for v in 0 .. 256 { let input = (v as u128) << (shift * 8) ; let hasher = constructor (0 , 0) ; alternatives . push (hash_with (& input , hasher)) ; } assert_each_byte_differs (shift , base , alternatives) ; } }
};
}
