macro_rules! deps {
    () => {
        ModeAuthPsk!();
    };
}

macro_rules! impl_574 {
    () => {
        deps!();
        impl < S > ModeAuthPsk < S > { # [doc = " HPKE AuthPsk mode ID."] pub const MODE_ID : u8 = 0x03u8 ; }
    };
}

impl_574!();