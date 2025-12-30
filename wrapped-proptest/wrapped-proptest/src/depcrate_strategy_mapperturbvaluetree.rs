// Generated macro for PerturbValueTree (struct)
macro_rules! Depcrate_strategy_mapPerturbValueTree {
() => {
// Module: crate::strategy::map
// Provides: {"PerturbValueTree"}
// Dependencies: {}
# [doc = " `ValueTree` perturbation adaptor."] # [doc = ""] # [doc = " See `Strategy::prop_perturb()`."] pub struct PerturbValueTree < S , F > { source : S , fun : Arc < F > , rng : TestRng , }
};
}
