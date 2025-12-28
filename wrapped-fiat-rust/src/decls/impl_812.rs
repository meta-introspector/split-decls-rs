macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! impl_812 {
    () => {
        deps!();
        impl < 'a > IndexConst < & 'a fiat_secp256k1_montgomery_montgomery_domain_field_element > { # [allow (unused)] # [inline (always)] const fn index (self , i : usize) -> & 'a u64 { & self . 0 . 0 [i] } }
    };
}

impl_812!()