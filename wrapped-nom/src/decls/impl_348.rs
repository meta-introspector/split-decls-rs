macro_rules! deps {
    () => {
        FindToken!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl < const N : usize > FindToken < u8 > for [u8 ; N] { fn find_token (& self , token : u8) -> bool { memchr :: memchr (token , & self [..]) . is_some () } }
    };
}

impl_348!()