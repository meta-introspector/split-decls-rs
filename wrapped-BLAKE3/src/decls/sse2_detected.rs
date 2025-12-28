macro_rules! sse2_detected {
    () => {
        # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] # [inline (always)] pub fn sse2_detected () -> bool { if cfg ! (miri) { return false ; } if cfg ! (feature = "no_sse2") { return false ; } cpufeatures :: new ! (has_sse2 , "sse2") ; has_sse2 :: get () }
    };
}

sse2_detected!();