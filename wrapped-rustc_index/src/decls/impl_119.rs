macro_rules! deps {
    () => {
        Idx!();
        IndexVec!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < 'a , I : Idx , T > IntoIterator for & 'a mut IndexVec < I , T > { type Item = & 'a mut T ; type IntoIter = slice :: IterMut < 'a , T > ; # [inline] fn into_iter (self) -> slice :: IterMut < 'a , T > { self . iter_mut () } }
    };
}

impl_119!();