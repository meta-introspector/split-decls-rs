// Generated macro for SignedRoundingMode (enum)
macro_rules! Depcrate_roundingSignedRoundingMode {
() => {
// Module: crate::rounding
// Provides: {"SignedRoundingMode"}
// Dependencies: {}
# [doc = " Mode used in a signed rounding operations."] # [doc = ""] # [doc = " NOTE:"] # [doc = "   - You can find the comparative table of all the rounding modes in the [`UnsignedRoundingMode`] documentation."] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] # [non_exhaustive] pub enum SignedRoundingMode { Unsigned (UnsignedRoundingMode) , Ceil , Floor , HalfCeil , HalfFloor , }
};
}
