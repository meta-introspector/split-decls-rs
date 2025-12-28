macro_rules! avx2_detected {
    () => {
        # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] # [inline (always)] pub fn avx2_detected () -> bool { if cfg ! (miri) { return false ; } if cfg ! (feature = "no_avx2") { return false ; } cpufeatures :: new ! (has_avx2 , "avx2") ; has_avx2 :: get () }
    };
}

avx2_detected!();