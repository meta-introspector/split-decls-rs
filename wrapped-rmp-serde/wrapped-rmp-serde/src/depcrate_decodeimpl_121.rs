// Generated macro for impl_121 (impl)
macro_rules! Depcrate_decodeimpl_121 {
() => {
// Module: crate::decode
// Provides: {"impl_121"}
// Dependencies: {}
impl < T : AsRef < [u8] > + ? Sized > Read for ReadRefReader < '_ , T > { # [inline] fn read (& mut self , buf : & mut [u8]) -> Result < usize , io :: Error > { self . buf . read (buf) } # [inline] fn read_exact (& mut self , buf : & mut [u8]) -> Result < () , io :: Error > { self . buf . read_exact (buf) } }
};
}
