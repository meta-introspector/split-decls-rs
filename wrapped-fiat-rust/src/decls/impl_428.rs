macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        impl < 'a > IndexConst < & 'a fiat_p384_non_montgomery_domain_field_element > { # [allow (unused)] # [inline (always)] const fn index (self , i : usize) -> & 'a u64 { & self . 0 . 0 [i] } }
    };
}

impl_428!();