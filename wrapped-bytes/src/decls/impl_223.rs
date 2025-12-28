macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl PartialOrd < str > for BytesMut { fn partial_cmp (& self , other : & str) -> Option < cmp :: Ordering > { (* * self) . partial_cmp (other . as_bytes ()) } }
    };
}

impl_223!()