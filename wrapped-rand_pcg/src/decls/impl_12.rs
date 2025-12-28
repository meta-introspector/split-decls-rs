macro_rules! deps {
    () => {
        Mcg128Xsl64!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl RngCore for Mcg128Xsl64 { # [inline] fn next_u32 (& mut self) -> u32 { self . next_u64 () as u32 } # [inline] fn next_u64 (& mut self) -> u64 { self . state = self . state . wrapping_mul (MULTIPLIER) ; output_xsl_rr (self . state) } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { le :: fill_bytes_via_next (self , dest) } }
    };
}

impl_12!();