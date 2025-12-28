macro_rules! deps {
    () => {
        Error!();
        Vectorization!();
    };
}

macro_rules! hex_encode_custom {
    () => {
        deps!();
        pub fn hex_encode_custom < 'a > (src : & [u8] , dst : & 'a mut [u8] , upper_case : bool ,) -> Result < & 'a mut str , Error > { unsafe fn mut_str (buffer : & mut [u8]) -> & mut str { if cfg ! (debug_assertions) { core :: str :: from_utf8_mut (buffer) . unwrap () } else { core :: str :: from_utf8_unchecked_mut (buffer) } } let expect_dst_len = src . len () . checked_mul (2) . ok_or (Error :: InvalidLength (src . len ())) ? ; if dst . len () < expect_dst_len { return Err (Error :: InvalidLength (expect_dst_len)) ; } # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { match crate :: vectorization_support () { crate :: Vectorization :: AVX2 => unsafe { hex_encode_avx2 (src , dst , upper_case) } , crate :: Vectorization :: SSE41 => unsafe { hex_encode_sse41 (src , dst , upper_case) } , crate :: Vectorization :: None => hex_encode_custom_case_fallback (src , dst , upper_case) , } return Ok (unsafe { mut_str (dst) }) ; } # [cfg (target_arch = "aarch64")] { match crate :: vectorization_support () { crate :: Vectorization :: Neon => unsafe { hex_encode_neon (src , dst , upper_case) } , crate :: Vectorization :: None => hex_encode_custom_case_fallback (src , dst , upper_case) , } return Ok (unsafe { mut_str (dst) }) ; } # [cfg (not (any (target_arch = "x86" , target_arch = "x86_64" , target_arch = "aarch64")))] { hex_encode_custom_case_fallback (src , dst , upper_case) ; Ok (unsafe { mut_str (dst) }) } }
    };
}

hex_encode_custom!();