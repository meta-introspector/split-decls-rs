// Generated macro for test_length_extension (function)
macro_rules! Depcrate_hash_quality_testtest_length_extension {
() => {
// Module: crate::hash_quality_test
// Provides: {"test_length_extension"}
// Dependencies: {}
fn test_length_extension < T : Hasher > (hasher : impl Fn (u128 , u128) -> T) { for key in 0 .. 256 { let h1 = hasher (key , key) ; let v1 = hash_with (& [0_u8 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , h1) ; let h2 = hasher (key , key) ; let v2 = hash_with (& [1_u8 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , h2) ; assert_ne ! (v1 , v2) ; } }
};
}
