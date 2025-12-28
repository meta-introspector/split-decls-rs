macro_rules! deps {
    () => {
        IntoIter!();
        BytesMut!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl IntoIterator for BytesMut { type Item = u8 ; type IntoIter = IntoIter < BytesMut > ; fn into_iter (self) -> Self :: IntoIter { IntoIter :: new (self) } }
    };
}

impl_203!()