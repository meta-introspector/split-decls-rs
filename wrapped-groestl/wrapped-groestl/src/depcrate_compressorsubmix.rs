// Generated macro for submix (function)
macro_rules! Depcrate_compressorsubmix {
() => {
// Module: crate::compressor
// Provides: {"submix"}
// Dependencies: {}
# [doc = " Combined subtract and mix; common to Large and Small variants."] # [inline (always)] unsafe fn submix (a : X8) -> X8 { let b0 = _mm_cvtsi64_si128 (0) ; let a = a . map (| x | _mm_aesenclast_si128 (x , b0)) ; let t = a ^ a . rotl1 () ; let b = a . rotl2 () ^ t . rotl4 () ^ t . rotl6 () ; let a = t ^ t . rotl3 () ; let a = a . map (mul2) ^ b ; b ^ a . rotl3 () . map (mul2) }
};
}
