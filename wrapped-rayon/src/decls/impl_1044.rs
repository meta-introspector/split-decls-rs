macro_rules! deps {
    () => {
        Iter!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_1044 {
    () => {
        deps!();
        impl < 'a , T : Sync > IntoParallelIterator for & 'a Option < T > { type Item = & 'a T ; type Iter = Iter < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { Iter { inner : self . as_ref () . into_par_iter () , } } }
    };
}

impl_1044!()