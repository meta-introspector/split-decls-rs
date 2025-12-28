macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        impl PartialOrd < BytesMut > for str { fn partial_cmp (& self , other : & BytesMut) -> Option < cmp :: Ordering > { < [u8] as PartialOrd < [u8] > > :: partial_cmp (self . as_bytes () , other) } }
    };
}

impl_225!()