// Generated macro for impl_29 (impl)
macro_rules! Depcrate_pcg128cmimpl_29 {
() => {
// Module: crate::pcg128cm
// Provides: {"impl_29"}
// Dependencies: {}
impl RngCore for Lcg128CmDxsm64 { # [inline] fn next_u32 (& mut self) -> u32 { self . next_u64 () as u32 } # [inline] fn next_u64 (& mut self) -> u64 { let res = output_dxsm (self . state) ; self . step () ; res } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { le :: fill_bytes_via_next (self , dest) } }
};
}
