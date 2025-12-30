// Generated macro for test_all_bytes_matter (function)
macro_rules! Depcrate_hash_quality_testtest_all_bytes_matter {
() => {
// Module: crate::hash_quality_test
// Provides: {"test_all_bytes_matter"}
// Dependencies: {}
fn test_all_bytes_matter < T : Hasher > (hasher : impl Fn () -> T) { let mut item = vec ! [0 ; 256] ; let base_hash = hash (& item , & hasher) ; for pos in 0 .. 256 { item [pos] = 255 ; let hash = hash (& item , & hasher) ; assert_ne ! (base_hash , hash , "Position {} did not affect output" , pos) ; item [pos] = 0 ; } }
};
}
