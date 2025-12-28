macro_rules! RelocBlock {
    () => {
        struct RelocBlock { virtual_address : u32 , count : u32 , }
    };
}

RelocBlock!();