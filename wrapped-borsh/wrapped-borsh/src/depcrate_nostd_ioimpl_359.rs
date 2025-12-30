// Generated macro for impl_359 (impl)
macro_rules! Depcrate_nostd_ioimpl_359 {
() => {
// Module: crate::nostd_io
// Provides: {"impl_359"}
// Dependencies: {}
impl < R : Read + ? Sized > Read for & mut R { # [inline] fn read (& mut self , buf : & mut [u8]) -> Result < usize > { (* * self) . read (buf) } # [inline] fn read_exact (& mut self , buf : & mut [u8]) -> Result < () > { (* * self) . read_exact (buf) } }
};
}
