// Generated macro for NonZeroScalar (type)
macro_rules! DepcrateNonZeroScalar {
() => {
// Module: crate
// Provides: {"NonZeroScalar"}
// Dependencies: {}
# [doc = " Non-zero NIST P-521 scalar field element."] # [cfg (feature = "arithmetic")] pub type NonZeroScalar = elliptic_curve :: NonZeroScalar < NistP521 > ;
};
}
