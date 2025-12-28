macro_rules! deps {
    () => {
        FindToken!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl < 'a , 'b > FindToken < & 'a u8 > for & 'b str { fn find_token (& self , token : & u8) -> bool { self . as_bytes () . find_token (token) } }
    };
}

impl_334!();