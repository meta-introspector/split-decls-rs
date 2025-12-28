macro_rules! deps {
    () => {
        FindToken!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl < 'a > FindToken < char > for & 'a [u8] { fn find_token (& self , token : char) -> bool { self . iter () . any (| i | * i == token as u8) } }
    };
}

impl_335!()