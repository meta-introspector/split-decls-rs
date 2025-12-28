macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! impl_607 {
    () => {
        deps!();
        impl < 'a > IndexConst < & 'a fiat_p448_tight_field_element > { # [allow (unused)] # [inline (always)] const fn index (self , i : usize) -> & 'a u64 { & self . 0 . 0 [i] } }
    };
}

impl_607!();