macro_rules! deps {
    () => {
        Iter!();
        IntoParallelIterator!();
        IterMut!();
    };
}

macro_rules! impl_1244 {
    () => {
        deps!();
        impl < 'data , T : Send > IntoParallelIterator for & 'data mut Box < [T] > { type Item = & 'data mut T ; type Iter = IterMut < 'data , T > ; fn into_par_iter (self) -> Self :: Iter { IterMut { slice : self } } }
    };
}

impl_1244!();