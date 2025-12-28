macro_rules! deps {
    () => {
        IntoIter!();
        Iter!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < T : Send > IntoParallelIterator for VecDeque < T > { type Item = T ; type Iter = IntoIter < T > ; fn into_par_iter (self) -> Self :: Iter { let inner = Vec :: from (self) . into_par_iter () ; IntoIter { inner } } }
    };
}

impl_94!();