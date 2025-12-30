// Generated macro for NEG_G1_GENERATOR_AFFINE (static)
macro_rules! Depcrate_pubkey_pointsNEG_G1_GENERATOR_AFFINE {
() => {
// Module: crate::pubkey::points
// Provides: {"NEG_G1_GENERATOR_AFFINE"}
// Dependencies: {}
# [cfg (all (not (target_os = "solana") , feature = "std"))] pub (crate) static NEG_G1_GENERATOR_AFFINE : LazyLock < G1Affine > = LazyLock :: new (| | (- G1Projective :: generator ()) . into ()) ;
};
}
