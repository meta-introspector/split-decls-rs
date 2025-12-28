macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl PartialOrd < Vec < u8 > > for Bytes { fn partial_cmp (& self , other : & Vec < u8 >) -> Option < cmp :: Ordering > { self . as_slice () . partial_cmp (& other [..]) } }
    };
}

impl_93!();