macro_rules! deps {
    () => {
        Vectorization!();
    };
}

macro_rules! hex_decode_unchecked {
    () => {
        deps!();
        pub fn hex_decode_unchecked (src : & [u8] , dst : & mut [u8]) { # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { match crate :: vectorization_support () { crate :: Vectorization :: AVX2 => unsafe { hex_decode_avx2 (src , dst) } , crate :: Vectorization :: None | crate :: Vectorization :: SSE41 => { hex_decode_fallback (src , dst) } } } # [cfg (not (any (target_arch = "x86" , target_arch = "x86_64")))] hex_decode_fallback (src , dst) ; }
    };
}

hex_decode_unchecked!();