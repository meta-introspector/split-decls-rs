macro_rules! deps {
    () => {
        Hash!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl fmt :: Debug for Hash { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let hex = self . to_hex () ; let hex : & str = hex . as_str () ; f . debug_tuple ("Hash") . field (& hex) . finish () } }
    };
}

impl_178!();