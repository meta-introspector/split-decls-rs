macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl PartialOrd < [u8] > for Bytes { fn partial_cmp (& self , other : & [u8]) -> Option < cmp :: Ordering > { self . as_slice () . partial_cmp (other) } }
    };
}

impl_85!();