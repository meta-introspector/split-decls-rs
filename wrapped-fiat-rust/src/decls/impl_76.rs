macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < 'a > IndexConst < & 'a fiat_25519_scalar_montgomery_domain_field_element > { # [allow (unused)] # [inline (always)] const fn index (self , i : usize) -> & 'a u32 { & self . 0 . 0 [i] } }
    };
}

impl_76!()