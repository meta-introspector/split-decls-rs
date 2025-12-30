// Generated macro for impl_41 (impl)
macro_rules! Depcrate_pcg64impl_41 {
() => {
// Module: crate::pcg64
// Provides: {"impl_41"}
// Dependencies: {}
impl RngCore for Lcg64Xsh32 { # [inline] fn next_u32 (& mut self) -> u32 { let state = self . state ; self . step () ; const ROTATE : u32 = 59 ; const XSHIFT : u32 = 18 ; const SPARE : u32 = 27 ; let rot = (state >> ROTATE) as u32 ; let xsh = (((state >> XSHIFT) ^ state) >> SPARE) as u32 ; xsh . rotate_right (rot) } # [inline] fn next_u64 (& mut self) -> u64 { le :: next_u64_via_u32 (self) } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { le :: fill_bytes_via_next (self , dest) } }
};
}
