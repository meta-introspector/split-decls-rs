macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl PartialOrd < BytesMut > for String { fn partial_cmp (& self , other : & BytesMut) -> Option < cmp :: Ordering > { < [u8] as PartialOrd < [u8] > > :: partial_cmp (self . as_bytes () , other) } }
    };
}

impl_233!()