// Generated macro for vec_random_btreeset (function)
macro_rules! Depcrate_membershipvec_random_btreeset {
() => {
// Module: crate::membership
// Provides: {"vec_random_btreeset"}
// Dependencies: {}
fn vec_random_btreeset (rng : & mut ThreadRng) -> Vec < BTreeSet < NodeId > > { let mut vec = Vec :: with_capacity (10) ; for _ in 0 .. rng . gen_range (0 .. 10) { vec . push (random_btreeset (rng)) ; } vec }
};
}
