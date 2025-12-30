// Generated macro for expand_a (function)
macro_rules! Depcrate_samplingexpand_a {
() => {
// Module: crate::sampling
// Provides: {"expand_a"}
// Dependencies: {}
pub (crate) fn expand_a < K : ArraySize , L : ArraySize > (rho : & [u8]) -> NttMatrix < K , L > { NttMatrix :: new (Array :: from_fn (| r | { NttVector :: new (Array :: from_fn (| s | { rej_ntt_poly (rho , Truncate :: truncate (r) , Truncate :: truncate (s)) })) })) }
};
}
