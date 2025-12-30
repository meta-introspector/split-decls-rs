// Generated macro for impl_316 (impl)
macro_rules! Depcrate_rngs_reseedingimpl_316 {
() => {
// Module: crate::rngs::reseeding
// Provides: {"impl_316"}
// Dependencies: {}
impl < R , Rsdr > RngCore for ReseedingRng < R , Rsdr > where R : BlockRngCore < Item = u32 > + SeedableRng , Rsdr : TryRngCore , { # [inline (always)] fn next_u32 (& mut self) -> u32 { self . 0 . next_u32 () } # [inline (always)] fn next_u64 (& mut self) -> u64 { self . 0 . next_u64 () } fn fill_bytes (& mut self , dest : & mut [u8]) { self . 0 . fill_bytes (dest) } }
};
}
