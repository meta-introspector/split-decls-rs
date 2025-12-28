macro_rules! avx2 {
    () => {
        # [cfg (blake3_avx2_ffi)] # [path = "ffi_avx2.rs"] mod avx2 ;
    };
}

avx2!();