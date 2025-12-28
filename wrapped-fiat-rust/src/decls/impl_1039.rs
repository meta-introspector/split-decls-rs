macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! impl_1039 {
    () => {
        deps!();
        impl < 'a > IndexConst < & 'a fiat_sm2_scalar_non_montgomery_domain_field_element > { # [allow (unused)] # [inline (always)] const fn index (self , i : usize) -> & 'a u64 { & self . 0 . 0 [i] } }
    };
}

impl_1039!()