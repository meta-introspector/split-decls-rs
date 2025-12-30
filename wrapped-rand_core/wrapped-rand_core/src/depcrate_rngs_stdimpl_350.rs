// Generated macro for impl_350 (impl)
macro_rules! Depcrate_rngs_stdimpl_350 {
() => {
// Module: crate::rngs::std
// Provides: {"impl_350"}
// Dependencies: {}
impl RngCore for StdRng { # [inline (always)] fn next_u32 (& mut self) -> u32 { self . 0 . next_u32 () } # [inline (always)] fn next_u64 (& mut self) -> u64 { self . 0 . next_u64 () } # [inline (always)] fn fill_bytes (& mut self , dst : & mut [u8]) { self . 0 . fill_bytes (dst) } }
};
}
