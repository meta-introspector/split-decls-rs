macro_rules! deps {
    () => {
        SmallRng!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl RngCore for SmallRng { # [inline (always)] fn next_u32 (& mut self) -> u32 { self . 0 . next_u32 () } # [inline (always)] fn next_u64 (& mut self) -> u64 { self . 0 . next_u64 () } # [inline (always)] fn fill_bytes (& mut self , dest : & mut [u8]) { self . 0 . fill_bytes (dest) } }
    };
}

impl_232!()