macro_rules! deps {
    () => {
        BytesMut!();
        IntoIter!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a BytesMut { type Item = & 'a u8 ; type IntoIter = core :: slice :: Iter < 'a , u8 > ; fn into_iter (self) -> Self :: IntoIter { self . as_ref () . iter () } }
    };
}

impl_204!();