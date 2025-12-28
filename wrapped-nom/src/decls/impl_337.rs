macro_rules! deps {
    () => {
        FindToken!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        impl < 'a > FindToken < char > for & 'a [char] { fn find_token (& self , token : char) -> bool { self . iter () . any (| i | * i == token) } }
    };
}

impl_337!();