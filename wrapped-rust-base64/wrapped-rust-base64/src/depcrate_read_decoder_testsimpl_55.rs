// Generated macro for impl_55 (impl)
macro_rules! Depcrate_read_decoder_testsimpl_55 {
() => {
// Module: crate::read::decoder_tests
// Provides: {"impl_55"}
// Dependencies: {}
impl < R : io :: Read > io :: Read for ShortRead < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let len = self . max_read_len . max (buf . len ()) ; self . delegate . read (& mut buf [.. len]) } }
};
}
