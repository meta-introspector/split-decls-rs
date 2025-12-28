macro_rules! deps {
    () => {
        FindToken!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        impl < 'a > FindToken < char > for & 'a str { fn find_token (& self , token : char) -> bool { self . chars () . any (| i | i == token) } }
    };
}

impl_336!()