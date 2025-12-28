macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl PartialOrd < Bytes > for Vec < u8 > { fn partial_cmp (& self , other : & Bytes) -> Option < cmp :: Ordering > { < [u8] as PartialOrd < [u8] > > :: partial_cmp (self , other) } }
    };
}

impl_95!();