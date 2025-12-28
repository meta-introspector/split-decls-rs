macro_rules! deps {
    () => {
        IntoIter!();
        Iter!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_1347 {
    () => {
        deps!();
        impl < T : Send > IntoParallelIterator for Vec < T > { type Item = T ; type Iter = IntoIter < T > ; fn into_par_iter (self) -> Self :: Iter { IntoIter { vec : self } } }
    };
}

impl_1347!()