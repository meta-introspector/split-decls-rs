macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl PartialOrd < Bytes > for str { fn partial_cmp (& self , other : & Bytes) -> Option < cmp :: Ordering > { < [u8] as PartialOrd < [u8] > > :: partial_cmp (self . as_bytes () , other) } }
    };
}

impl_91!();