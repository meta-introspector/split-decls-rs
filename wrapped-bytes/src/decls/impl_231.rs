macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl PartialOrd < String > for BytesMut { fn partial_cmp (& self , other : & String) -> Option < cmp :: Ordering > { (* * self) . partial_cmp (other . as_bytes ()) } }
    };
}

impl_231!()