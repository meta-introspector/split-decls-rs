macro_rules! sse2 {
    () => {
        # [cfg (all (any (target_arch = "x86" , target_arch = "x86_64") , target_feature = "sse2" , not (target_feature = "avx") , not (target_feature = "avx2") ,))] mod sse2 ;
    };
}

sse2!()