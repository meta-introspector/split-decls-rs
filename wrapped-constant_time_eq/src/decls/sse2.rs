macro_rules! sse2 {
    () => {
        # [cfg (all (any (target_arch = "x86" , target_arch = "x86_64") , target_feature = "sse2" , not (miri)))] mod sse2 ;
    };
}

sse2!()