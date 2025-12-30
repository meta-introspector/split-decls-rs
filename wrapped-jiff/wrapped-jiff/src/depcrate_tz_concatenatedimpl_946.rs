// Generated macro for impl_946 (impl)
macro_rules! Depcrate_tz_concatenatedimpl_946 {
() => {
// Module: crate::tz::concatenated
// Provides: {"impl_946"}
// Dependencies: {}
impl < 'a , R : Read + ? Sized > Read for & 'a R { fn read_exact_at (& self , buf : & mut [u8] , offset : u64) -> Result < () , Error > { (* * self) . read_exact_at (buf , offset) } }
};
}
