macro_rules! deps {
    () => {
        FindToken!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl < 'a > FindToken < u8 > for & 'a str { fn find_token (& self , token : u8) -> bool { self . as_bytes () . find_token (token) } }
    };
}

impl_332!();