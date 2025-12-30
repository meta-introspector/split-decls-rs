// Generated macro for NonZeroScalar (type)
macro_rules! DepcrateNonZeroScalar {
() => {
// Module: crate
// Provides: {"NonZeroScalar"}
// Dependencies: {}
# [doc = " Non-zero secp256k1 (K-256) scalar field element."] # [cfg (feature = "arithmetic")] pub type NonZeroScalar = elliptic_curve :: NonZeroScalar < Secp256k1 > ;
};
}
