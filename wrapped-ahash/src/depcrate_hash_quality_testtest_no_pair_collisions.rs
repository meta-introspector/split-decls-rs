// Generated macro for test_no_pair_collisions (function)
macro_rules! Depcrate_hash_quality_testtest_no_pair_collisions {
() => {
// Module: crate::hash_quality_test
// Provides: {"test_no_pair_collisions"}
// Dependencies: {}
fn test_no_pair_collisions < T : Hasher > (hasher : impl Fn () -> T) { let base = [0_u64 , 0_u64] ; let base_hash = hash (& base , & hasher) ; for bitpos1 in 0 .. 64 { let a = 1_u64 << bitpos1 ; for bitpos2 in 0 .. bitpos1 { let b = 1_u64 << bitpos2 ; let aa = hash (& [a , a] , & hasher) ; let ab = hash (& [a , b] , & hasher) ; let ba = hash (& [b , a] , & hasher) ; let bb = hash (& [b , b] , & hasher) ; assert_sufficiently_different (base_hash , aa , 3) ; assert_sufficiently_different (base_hash , ab , 3) ; assert_sufficiently_different (base_hash , ba , 3) ; assert_sufficiently_different (base_hash , bb , 3) ; assert_sufficiently_different (aa , ab , 3) ; assert_sufficiently_different (ab , ba , 3) ; assert_sufficiently_different (ba , bb , 3) ; assert_sufficiently_different (aa , ba , 3) ; assert_sufficiently_different (ab , bb , 3) ; assert_sufficiently_different (aa , bb , 3) ; } } }
};
}
