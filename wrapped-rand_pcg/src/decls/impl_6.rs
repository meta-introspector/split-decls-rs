macro_rules! deps {
    () => {
        Lcg128Xsl64!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl RngCore for Lcg128Xsl64 { # [inline] fn next_u32 (& mut self) -> u32 { self . next_u64 () as u32 } # [inline] fn next_u64 (& mut self) -> u64 { self . step () ; output_xsl_rr (self . state) } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { le :: fill_bytes_via_next (self , dest) } }
    };
}

impl_6!()