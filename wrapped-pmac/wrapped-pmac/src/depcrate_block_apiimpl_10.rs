// Generated macro for impl_10 (impl)
macro_rules! Depcrate_block_apiimpl_10 {
() => {
// Module: crate::block_api
// Provides: {"impl_10"}
// Dependencies: {}
impl < C : PmacCipher , const LC_SIZE : usize > PmacState < C , LC_SIZE > { # [inline (always)] fn next_offset (& mut self) -> & Block < C > { let ntz = self . counter . trailing_zeros () as usize ; self . counter += 1 ; let l = if ntz < LC_SIZE { self . l_cache [ntz] . clone () } else { let mut block = self . l_cache [LC_SIZE - 1] . clone () ; for _ in LC_SIZE - 1 .. ntz { block = C :: dbl (block) ; } block } ; xor (& mut self . offset , & l) ; & self . offset } }
};
}
