// Generated macro for expand_mask (function)
macro_rules! Depcrate_samplingexpand_mask {
() => {
// Module: crate::sampling
// Provides: {"expand_mask"}
// Dependencies: {}
pub (crate) fn expand_mask < K , Gamma1 > (rho : & [u8] , mu : u16) -> Vector < K > where K : ArraySize , Gamma1 : MaskSamplingSize , { Vector :: new (Array :: from_fn (| r | { let r : u16 = Truncate :: truncate (r) ; let v = H :: default () . absorb (rho) . absorb (& (mu + r) . to_le_bytes ()) . squeeze_new :: < Gamma1 :: SampleSize > () ; Gamma1 :: unpack (& v) })) }
};
}
