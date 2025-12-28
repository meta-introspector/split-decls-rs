macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl < 'a > IndexConst < & 'a fiat_p256_montgomery_domain_field_element > { # [allow (unused)] # [inline (always)] const fn index (self , i : usize) -> & 'a u32 { & self . 0 . 0 [i] } }
    };
}

impl_238!()