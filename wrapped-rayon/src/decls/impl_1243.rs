macro_rules! deps {
    () => {
        IntoParallelIterator!();
        Iter!();
        IterMut!();
    };
}

macro_rules! impl_1243 {
    () => {
        deps!();
        impl < 'data , T : Send > IntoParallelIterator for & 'data mut [T] { type Item = & 'data mut T ; type Iter = IterMut < 'data , T > ; fn into_par_iter (self) -> Self :: Iter { IterMut { slice : self } } }
    };
}

impl_1243!()