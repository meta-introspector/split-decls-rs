macro_rules! deps {
    () => {
        FindToken!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        impl < 'a , 'b > FindToken < & 'a u8 > for & 'b [u8] { fn find_token (& self , token : & u8) -> bool { self . find_token (* token) } }
    };
}

impl_333!();