// Generated macro for random_btreeset (function)
macro_rules! Depcrate_membershiprandom_btreeset {
() => {
// Module: crate::membership
// Provides: {"random_btreeset"}
// Dependencies: {}
fn random_btreeset (rng : & mut ThreadRng) -> BTreeSet < NodeId > { let mut set = BTreeSet :: new () ; for _ in 0 .. rng . gen_range (0 .. 100) { let v = rng . gen () ; set . insert (v) ; } set }
};
}
