macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl PartialOrd < Bytes > for [u8] { fn partial_cmp (& self , other : & Bytes) -> Option < cmp :: Ordering > { < [u8] as PartialOrd < [u8] > > :: partial_cmp (self , other) } }
    };
}

impl_87!();