macro_rules! neon {
    () => {
        # [cfg (blake3_neon)] # [path = "ffi_neon.rs"] mod neon ;
    };
}

neon!();