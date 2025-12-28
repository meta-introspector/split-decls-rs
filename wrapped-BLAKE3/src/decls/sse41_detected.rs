macro_rules! sse41_detected {
    () => {
        # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] # [inline (always)] pub fn sse41_detected () -> bool { if cfg ! (miri) { return false ; } if cfg ! (feature = "no_sse41") { return false ; } cpufeatures :: new ! (has_sse41 , "sse4.1") ; has_sse41 :: get () }
    };
}

sse41_detected!()