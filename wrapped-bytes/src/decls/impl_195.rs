macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl Ord for BytesMut { fn cmp (& self , other : & BytesMut) -> cmp :: Ordering { self . as_slice () . cmp (other . as_slice ()) } }
    };
}

impl_195!();