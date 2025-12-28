macro_rules! deps {
    () => {
        IndexVec!();
        Idx!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < 'a , I : Idx , T > IntoIterator for & 'a IndexVec < I , T > { type Item = & 'a T ; type IntoIter = slice :: Iter < 'a , T > ; # [inline] fn into_iter (self) -> slice :: Iter < 'a , T > { self . iter () } }
    };
}

impl_118!();