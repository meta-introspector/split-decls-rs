// Generated macro for impl_322 (impl)
macro_rules! Depcrate_rngs_threadimpl_322 {
() => {
// Module: crate::rngs::thread
// Provides: {"impl_322"}
// Dependencies: {}
impl RngCore for ThreadRng { # [inline (always)] fn next_u32 (& mut self) -> u32 { let rng = unsafe { & mut * self . rng . get () } ; rng . next_u32 () } # [inline (always)] fn next_u64 (& mut self) -> u64 { let rng = unsafe { & mut * self . rng . get () } ; rng . next_u64 () } # [inline (always)] fn fill_bytes (& mut self , dest : & mut [u8]) { let rng = unsafe { & mut * self . rng . get () } ; rng . fill_bytes (dest) } }
};
}
