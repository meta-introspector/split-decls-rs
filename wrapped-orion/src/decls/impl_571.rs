macro_rules! deps {
    () => {
        ModeAuth!();
    };
}

macro_rules! impl_571 {
    () => {
        deps!();
        impl < S > ModeAuth < S > { # [doc = " HPKE Auth mode ID."] pub const MODE_ID : u8 = 0x02u8 ; }
    };
}

impl_571!()