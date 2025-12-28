macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl PartialOrd < BytesMut > for & [u8] { fn partial_cmp (& self , other : & BytesMut) -> Option < cmp :: Ordering > { < [u8] as PartialOrd < [u8] > > :: partial_cmp (self , other) } }
    };
}

impl_237!()