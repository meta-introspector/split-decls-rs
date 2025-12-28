macro_rules! avx512 {
    () => {
        # [cfg (blake3_avx512_ffi)] # [path = "ffi_avx512.rs"] mod avx512 ;
    };
}

avx512!()