macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! impl_671 {
    () => {
        deps!();
        impl < 'a > IndexConst < & 'a fiat_p521_tight_field_element > { # [allow (unused)] # [inline (always)] const fn index (self , i : usize) -> & 'a u64 { & self . 0 . 0 [i] } }
    };
}

impl_671!()