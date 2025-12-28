macro_rules! fpu_precision {
    () => {
        # [cfg (any (not (target_arch = "x86") , target_feature = "sse2"))] mod fpu_precision { pub fn set_precision < T > () { } }
    };
}

fpu_precision!();