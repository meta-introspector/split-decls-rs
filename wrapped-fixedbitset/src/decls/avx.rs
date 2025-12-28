macro_rules! avx {
    () => {
        # [cfg (all (any (target_arch = "x86" , target_arch = "x86_64") , target_feature = "avx" , not (target_feature = "avx2")))] mod avx ;
    };
}

avx!()