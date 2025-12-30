// Generated macro for MaskSamplingSize (trait)
macro_rules! Depcrate_paramMaskSamplingSize {
() => {
// Module: crate::param
// Provides: {"MaskSamplingSize"}
// Dependencies: {}
# [doc = " An integer that describes a mask sampling size"] # [expect (unreachable_pub)] pub trait MaskSamplingSize : Unsigned { type SampleSize : ArraySize ; fn unpack (v : & Array < u8 , Self :: SampleSize >) -> Polynomial ; }
};
}
