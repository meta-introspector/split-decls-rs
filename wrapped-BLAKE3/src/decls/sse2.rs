macro_rules! sse2 {
    () => {
        # [cfg (blake3_sse2_ffi)] # [path = "ffi_sse2.rs"] mod sse2 ;
    };
}

sse2!()