// Generated macro for impl_53 (impl)
macro_rules! Depcrate_read_decoder_testsimpl_53 {
() => {
// Module: crate::read::decoder_tests
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'a , 'b , R : io :: Read , N : rand :: Rng > io :: Read for RandomShortRead < 'a , 'b , R , N > { fn read (& mut self , buf : & mut [u8]) -> Result < usize , io :: Error > { let effective_len = cmp :: min (self . rng . gen_range (1 .. 20) , buf . len ()) ; self . delegate . read (& mut buf [.. effective_len]) } }
};
}
