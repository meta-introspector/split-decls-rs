macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! impl_892 {
    () => {
        deps!();
        impl < 'a , 'b > IndexConst < & 'a mut & 'b mut fiat_secp256k1_montgomery_scalar_non_montgomery_domain_field_element > { # [allow (unused)] # [inline (always)] const fn index_mut (self , i : usize) -> & 'a mut u64 { & mut self . 0 . 0 [i] } }
    };
}

impl_892!();