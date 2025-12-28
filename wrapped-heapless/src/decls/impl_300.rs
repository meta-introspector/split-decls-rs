macro_rules! deps {
    () => {
        VecInner!();
        LenType!();
        IntoIter!();
        Iter!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        impl < 'a , T , LenT : LenType , S : VecStorage < T > + ? Sized > IntoIterator for & 'a VecInner < T , LenT , S > { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_300!()