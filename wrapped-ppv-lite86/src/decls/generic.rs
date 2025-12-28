macro_rules! generic {
    () => {
        # [cfg (any (feature = "no_simd" , miri , not (target_arch = "x86_64") , all (target_arch = "x86_64" , not (target_feature = "sse2"))))] pub mod generic ;
    };
}

generic!();