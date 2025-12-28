macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl PartialOrd < Vec < u8 > > for BytesMut { fn partial_cmp (& self , other : & Vec < u8 >) -> Option < cmp :: Ordering > { (* * self) . partial_cmp (& other [..]) } }
    };
}

impl_227!()