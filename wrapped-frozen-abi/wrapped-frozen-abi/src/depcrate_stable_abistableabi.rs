// Generated macro for StableAbi (trait)
macro_rules! Depcrate_stable_abiStableAbi {
() => {
// Module: crate::stable_abi
// Provides: {"StableAbi"}
// Dependencies: {}
pub trait StableAbi : Sized { fn random (rng : & mut impl RngCore) -> Self where StandardUniform : rand :: distr :: Distribution < Self > , { rng . random :: < Self > () } }
};
}
