// Generated macro for from_splitmix (macro)
macro_rules! Depcrate_commonfrom_splitmix {
() => {
// Module: crate::common
// Provides: {"from_splitmix"}
// Dependencies: {}
# [doc = " Initialize a RNG from a `u64` seed using `SplitMix64`."] macro_rules ! from_splitmix { ($ seed : expr) => { { let mut rng = crate :: SplitMix64 :: seed_from_u64 ($ seed) ; Self :: from_rng (& mut rng) } } ; }
};
}
