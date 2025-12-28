macro_rules! deps {
    () => {
        ModePsk!();
    };
}

macro_rules! impl_568 {
    () => {
        deps!();
        impl < S > ModePsk < S > { # [doc = " HPKE Psk mode ID."] pub const MODE_ID : u8 = 0x01u8 ; }
    };
}

impl_568!();