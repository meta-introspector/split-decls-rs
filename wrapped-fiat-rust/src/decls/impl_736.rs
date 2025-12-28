macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! impl_736 {
    () => {
        deps!();
        impl < 'a , 'b > IndexConst < & 'a mut & 'b mut fiat_poly1305_tight_field_element > { # [allow (unused)] # [inline (always)] const fn index_mut (self , i : usize) -> & 'a mut u64 { & mut self . 0 . 0 [i] } }
    };
}

impl_736!();