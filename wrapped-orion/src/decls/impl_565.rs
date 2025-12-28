macro_rules! deps {
    () => {
        ModeBase!();
    };
}

macro_rules! impl_565 {
    () => {
        deps!();
        impl < S > ModeBase < S > { # [doc = " HPKE Base mode ID."] pub const MODE_ID : u8 = 0x00u8 ; }
    };
}

impl_565!();