// Generated macro for impl_11 (impl)
macro_rules! Depcrate_pcg128impl_11 {
() => {
// Module: crate::pcg128
// Provides: {"impl_11"}
// Dependencies: {}
impl RngCore for Lcg128Xsl64 { # [inline] fn next_u32 (& mut self) -> u32 { self . next_u64 () as u32 } # [inline] fn next_u64 (& mut self) -> u64 { self . step () ; output_xsl_rr (self . state) } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { le :: fill_bytes_via_next (self , dest) } }
};
}
