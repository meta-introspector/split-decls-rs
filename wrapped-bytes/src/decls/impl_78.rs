macro_rules! deps {
    () => {
        IntoIter!();
        Bytes!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a Bytes { type Item = & 'a u8 ; type IntoIter = core :: slice :: Iter < 'a , u8 > ; fn into_iter (self) -> Self :: IntoIter { self . as_slice () . iter () } }
    };
}

impl_78!()