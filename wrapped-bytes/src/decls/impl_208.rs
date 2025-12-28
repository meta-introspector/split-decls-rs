macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl FromIterator < u8 > for BytesMut { fn from_iter < T : IntoIterator < Item = u8 > > (into_iter : T) -> Self { BytesMut :: from_vec (Vec :: from_iter (into_iter)) } }
    };
}

impl_208!();