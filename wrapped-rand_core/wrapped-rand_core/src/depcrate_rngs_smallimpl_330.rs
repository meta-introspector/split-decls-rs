// Generated macro for impl_330 (impl)
macro_rules! Depcrate_rngs_smallimpl_330 {
() => {
// Module: crate::rngs::small
// Provides: {"impl_330"}
// Dependencies: {}
impl RngCore for SmallRng { # [inline (always)] fn next_u32 (& mut self) -> u32 { self . 0 . next_u32 () } # [inline (always)] fn next_u64 (& mut self) -> u64 { self . 0 . next_u64 () } # [inline (always)] fn fill_bytes (& mut self , dest : & mut [u8]) { self . 0 . fill_bytes (dest) } }
};
}
