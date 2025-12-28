macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < 'a > FromIterator < & 'a u8 > for BytesMut { fn from_iter < T : IntoIterator < Item = & 'a u8 > > (into_iter : T) -> Self { BytesMut :: from_iter (into_iter . into_iter () . copied ()) } }
    };
}

impl_209!()