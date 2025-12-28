macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < 'a , 'b > IndexConst < & 'a mut & 'b mut fiat_25519_loose_field_element > { # [allow (unused)] # [inline (always)] const fn index_mut (self , i : usize) -> & 'a mut u32 { & mut self . 0 . 0 [i] } }
    };
}

impl_11!();