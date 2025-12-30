// Generated macro for sse42_compile_time (module)
macro_rules! Depcrate_simdsse42_compile_time {
() => {
// Module: crate::simd
// Provides: {"sse42_compile_time"}
// Dependencies: {}
# [cfg (all (not (any (httparse_disable_simd , miri)) , target_feature = "sse4.2" , not (target_feature = "avx2") , any (target_arch = "x86" , target_arch = "x86_64" ,) ,))] mod sse42_compile_time { # [inline (always)] pub fn match_header_name_vectored (b : & mut crate :: iter :: Bytes < '_ >) { super :: swar :: match_header_name_vectored (b) ; } # [inline (always)] pub fn match_uri_vectored (b : & mut crate :: iter :: Bytes < '_ >) { unsafe { super :: sse42 :: match_uri_vectored (b) } } # [inline (always)] pub fn match_header_value_vectored (b : & mut crate :: iter :: Bytes < '_ >) { unsafe { super :: sse42 :: match_header_value_vectored (b) } } }
};
}
