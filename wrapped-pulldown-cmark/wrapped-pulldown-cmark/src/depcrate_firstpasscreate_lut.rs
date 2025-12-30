// Generated macro for create_lut (function)
macro_rules! Depcrate_firstpasscreate_lut {
() => {
// Module: crate::firstpass
// Provides: {"create_lut"}
// Dependencies: {}
fn create_lut (options : & Options) -> LookupTable { # [cfg (all (target_arch = "x86_64" , feature = "simd"))] { LookupTable { simd : simd :: compute_lookup (options) , scalar : special_bytes (options) , } } # [cfg (not (all (target_arch = "x86_64" , feature = "simd")))] { special_bytes (options) } }
};
}
