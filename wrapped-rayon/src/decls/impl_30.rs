macro_rules! deps {
    () => {
        Iter!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < 'a , T : Sync > IntoParallelIterator for & 'a BinaryHeap < T > { type Item = & 'a T ; type Iter = Iter < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { Iter { inner : self . as_slice () . into_par_iter () , } } }
    };
}

impl_30!();