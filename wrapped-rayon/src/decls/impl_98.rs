macro_rules! deps {
    () => {
        Iter!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < 'a , T : Sync > IntoParallelIterator for & 'a VecDeque < T > { type Item = & 'a T ; type Iter = Iter < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { let (a , b) = self . as_slices () ; Iter { inner : a . into_par_iter () . chain (b) , } } }
    };
}

impl_98!()