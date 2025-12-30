// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl < F > RngCore for JitterRng < F > where F : Fn () -> u64 + Send + Sync , { fn next_u32 (& mut self) -> u32 { if self . data_half_used { self . data_half_used = false ; (self . data >> 32) as u32 } else { self . data = self . next_u64 () ; self . data_half_used = true ; self . data as u32 } } fn next_u64 (& mut self) -> u64 { self . data_half_used = false ; self . gen_entropy () } fn fill_bytes (& mut self , dest : & mut [u8]) { le :: fill_bytes_via_next (self , dest) } }
};
}
