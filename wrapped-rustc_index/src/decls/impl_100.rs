macro_rules! deps {
    () => {
        Idx!();
        IndexSlice!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < 'a , I : Idx , T > IntoIterator for & 'a IndexSlice < I , T > { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; # [inline] fn into_iter (self) -> slice :: Iter < 'a , T > { self . raw . iter () } }
    };
}

impl_100!();