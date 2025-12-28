macro_rules! deps {
    () => {
        FindToken!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl < 'a , const N : usize > FindToken < & 'a u8 > for [u8 ; N] { fn find_token (& self , token : & u8) -> bool { self . find_token (* token) } }
    };
}

impl_349!();