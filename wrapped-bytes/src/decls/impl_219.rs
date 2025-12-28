macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl PartialOrd < [u8] > for BytesMut { fn partial_cmp (& self , other : & [u8]) -> Option < cmp :: Ordering > { (* * self) . partial_cmp (other) } }
    };
}

impl_219!()