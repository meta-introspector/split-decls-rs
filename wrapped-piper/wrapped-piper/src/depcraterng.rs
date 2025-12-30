// Generated macro for rng (function)
macro_rules! Depcraterng {
() => {
// Module: crate
// Provides: {"rng"}
// Dependencies: {}
# [doc = " Get a random number generator."] # [doc = ""] # [doc = " This uses a fixed seed due to the lack of a good RNG in `no_std` environments."] # [cfg (not (feature = "std"))] # [inline] fn rng () -> fastrand :: Rng { fastrand :: Rng :: with_seed (0x7e9b496634c97ec6) }
};
}
