macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'a > IndexConst < & 'a fiat_25519_loose_field_element > { # [allow (unused)] # [inline (always)] const fn index (self , i : usize) -> & 'a u32 { & self . 0 . 0 [i] } }
    };
}

impl_10!()