// Generated macro for ParameterSet (trait)
macro_rules! Depcrate_paramParameterSet {
() => {
// Module: crate::param
// Provides: {"ParameterSet"}
// Dependencies: {}
# [doc = " A `ParameterSet` captures the parameters that describe a particular instance of ML-DSA.  There"] # [doc = " are three variants, corresponding to three different security levels."] pub trait ParameterSet { # [doc = " Number of rows in the A matrix"] type K : ArraySize ; # [doc = " Number of columns in the A matrix"] type L : ArraySize ; # [doc = " Private key range"] type Eta : SamplingSize ; # [doc = " Error size bound for y"] type Gamma1 : MaskSamplingSize ; # [doc = " Low-order rounding range"] type Gamma2 : Unsigned ; # [doc = " Low-order rounding range (2 * gamma2 in terms of the spec)"] type TwoGamma2 : Unsigned ; # [doc = " Encoding width of the W1 polynomial, namely bitlen((q - 1) / (2 * gamma2) - 1)"] type W1Bits : EncodingSize ; # [doc = " Collision strength of `c_tilde`, in bytes (lambda / 4 in the spec)"] type Lambda : ArraySize ; # [doc = " Max number of true values in the hint"] type Omega : ArraySize ; # [doc = " Number of nonzero values in the polynomial c"] const TAU : usize ; # [doc = " Beta = Tau * Eta"] # [allow (clippy :: as_conversions)] # [allow (clippy :: cast_possible_truncation)] const BETA : u32 = (Self :: TAU as u32) * Self :: Eta :: U32 ; }
};
}
