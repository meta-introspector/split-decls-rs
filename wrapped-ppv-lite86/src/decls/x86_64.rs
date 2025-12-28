macro_rules! x86_64 {
    () => {
        # [cfg (all (target_arch = "x86_64" , target_feature = "sse2" , not (feature = "no_simd") , not (miri)))] pub mod x86_64 ;
    };
}

x86_64!();