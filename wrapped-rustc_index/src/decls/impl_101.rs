macro_rules! deps {
    () => {
        IndexSlice!();
        Idx!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < 'a , I : Idx , T > IntoIterator for & 'a mut IndexSlice < I , T > { type Item = & 'a mut T ; type IntoIter = slice :: IterMut < 'a , T > ; # [inline] fn into_iter (self) -> slice :: IterMut < 'a , T > { self . raw . iter_mut () } }
    };
}

impl_101!()