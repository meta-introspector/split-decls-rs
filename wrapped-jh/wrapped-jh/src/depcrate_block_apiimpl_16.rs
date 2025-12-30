// Generated macro for impl_16 (impl)
macro_rules! Depcrate_block_apiimpl_16 {
() => {
// Module: crate::block_api
// Provides: {"impl_16"}
// Dependencies: {}
impl Drop for JhCore { fn drop (& mut self) { # [cfg (feature = "zeroize")] { use digest :: zeroize :: Zeroize ; const N : usize = core :: mem :: size_of :: < Compressor > () ; unsafe { let p : * mut [u8 ; N] = (& mut self . state as * mut Compressor) . cast () ; core :: ptr :: write_volatile (p , [0u8 ; N]) ; } self . block_len . zeroize () ; } } }
};
}
