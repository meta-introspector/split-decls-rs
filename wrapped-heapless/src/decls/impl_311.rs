macro_rules! deps {
    () => {
        LenType!();
        IntoIter!();
        Vec!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        impl < T , LenT : LenType , const N : usize > IntoIterator for Vec < T , N , LenT > { type Item = T ; type IntoIter = IntoIter < T , N , LenT > ; fn into_iter (self) -> Self :: IntoIter { IntoIter { vec : self , next : LenT :: ZERO , } } }
    };
}

impl_311!();