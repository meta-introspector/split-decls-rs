// Generated macro for impl_334 (impl)
macro_rules! Depcrate_rngs_osimpl_334 {
() => {
// Module: crate::rngs::os
// Provides: {"impl_334"}
// Dependencies: {}
impl TryRngCore for OsRng { type Error = OsError ; # [inline] fn try_next_u32 (& mut self) -> Result < u32 , Self :: Error > { getrandom :: u32 () . map_err (OsError) } # [inline] fn try_next_u64 (& mut self) -> Result < u64 , Self :: Error > { getrandom :: u64 () . map_err (OsError) } # [inline] fn try_fill_bytes (& mut self , dest : & mut [u8]) -> Result < () , Self :: Error > { getrandom :: fill (dest) . map_err (OsError) } }
};
}
