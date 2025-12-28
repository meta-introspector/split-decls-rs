macro_rules! deps {
    () => {
        FindToken!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl < 'a , 'b > FindToken < & 'a char > for & 'b [char] { fn find_token (& self , token : & char) -> bool { self . find_token (* token) } }
    };
}

impl_338!()