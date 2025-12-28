macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl PartialOrd < BytesMut > for & str { fn partial_cmp (& self , other : & BytesMut) -> Option < cmp :: Ordering > { other . partial_cmp (self) } }
    };
}

impl_239!();