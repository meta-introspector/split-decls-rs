// Generated macro for Perturb (struct)
macro_rules! Depcrate_strategy_mapPerturb {
() => {
// Module: crate::strategy::map
// Provides: {"Perturb"}
// Dependencies: {}
# [doc = " `Strategy` perturbation adaptor."] # [doc = ""] # [doc = " See `Strategy::prop_perturb()`."] # [must_use = "strategies do nothing unless used"] pub struct Perturb < S , F > { pub (super) source : S , pub (super) fun : Arc < F > , }
};
}
