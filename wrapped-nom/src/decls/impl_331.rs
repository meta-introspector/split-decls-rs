macro_rules! deps {
    () => {
        FindToken!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        impl < 'a > FindToken < u8 > for & 'a [u8] { fn find_token (& self , token : u8) -> bool { memchr :: memchr (token , self) . is_some () } }
    };
}

impl_331!()