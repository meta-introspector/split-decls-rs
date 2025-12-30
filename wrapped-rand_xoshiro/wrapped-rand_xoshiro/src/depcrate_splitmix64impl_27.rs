// Generated macro for impl_27 (impl)
macro_rules! Depcrate_splitmix64impl_27 {
() => {
// Module: crate::splitmix64
// Provides: {"impl_27"}
// Dependencies: {}
impl RngCore for SplitMix64 { # [inline] fn next_u32 (& mut self) -> u32 { self . x = self . x . wrapping_add (PHI) ; let mut z = self . x ; z = (z ^ (z >> 33)) . wrapping_mul (0x62A9D9ED799705F5) ; z = (z ^ (z >> 28)) . wrapping_mul (0xCB24D0A5C88C35B3) ; (z >> 32) as u32 } # [inline] fn next_u64 (& mut self) -> u64 { self . x = self . x . wrapping_add (PHI) ; let mut z = self . x ; z = (z ^ (z >> 30)) . wrapping_mul (0xbf58476d1ce4e5b9) ; z = (z ^ (z >> 27)) . wrapping_mul (0x94d049bb133111eb) ; z ^ (z >> 31) } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
