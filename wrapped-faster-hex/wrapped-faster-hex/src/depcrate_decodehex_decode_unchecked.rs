// Generated macro for hex_decode_unchecked (function)
macro_rules! Depcrate_decodehex_decode_unchecked {
() => {
// Module: crate::decode
// Provides: {"hex_decode_unchecked"}
// Dependencies: {}
pub fn hex_decode_unchecked (src : & [u8] , dst : & mut [u8]) { # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { match crate :: vectorization_support () { crate :: Vectorization :: AVX2 => unsafe { hex_decode_avx2 (src , dst) } , crate :: Vectorization :: None | crate :: Vectorization :: SSE41 => { hex_decode_fallback (src , dst) } } } # [cfg (not (any (target_arch = "x86" , target_arch = "x86_64")))] hex_decode_fallback (src , dst) ; }
};
}
