macro_rules! deps {
    () => {
        Chain!();
        Buf!();
        IntoIter!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < T , U > IntoIterator for Chain < T , U > where T : Buf , U : Buf , { type Item = u8 ; type IntoIter = IntoIter < Chain < T , U > > ; fn into_iter (self) -> Self :: IntoIter { IntoIter :: new (self) } }
    };
}

impl_26!()