// Generated macro for hex_check_with_case (function)
macro_rules! Depcrate_decodehex_check_with_case {
() => {
// Module: crate::decode
// Provides: {"hex_check_with_case"}
// Dependencies: {}
# [doc = " Check if the input is valid hex bytes slice with case check"] pub fn hex_check_with_case (src : & [u8] , check_case : CheckCase) -> bool { # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { match crate :: vectorization_support () { crate :: Vectorization :: AVX2 | crate :: Vectorization :: SSE41 => unsafe { hex_check_sse_with_case (src , check_case) } , crate :: Vectorization :: None => hex_check_fallback_with_case (src , check_case) , } } # [cfg (target_arch = "aarch64")] { match crate :: vectorization_support () { crate :: Vectorization :: Neon => unsafe { hex_check_neon_with_case (src , check_case) } , crate :: Vectorization :: None => hex_check_fallback_with_case (src , check_case) , } } # [cfg (not (any (target_arch = "x86" , target_arch = "x86_64" , target_arch = "aarch64")))] hex_check_fallback_with_case (src , check_case) }
};
}
