macro_rules! avx2 {
    () => {
        # [cfg (all (any (target_arch = "x86" , target_arch = "x86_64") , target_feature = "avx2"))] mod avx2 ;
    };
}

avx2!();