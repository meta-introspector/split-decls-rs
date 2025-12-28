macro_rules! sse41 {
    () => {
        # [cfg (blake3_sse41_ffi)] # [path = "ffi_sse41.rs"] mod sse41 ;
    };
}

sse41!()