macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl PartialOrd < BytesMut > for Vec < u8 > { fn partial_cmp (& self , other : & BytesMut) -> Option < cmp :: Ordering > { other . partial_cmp (self) } }
    };
}

impl_229!()