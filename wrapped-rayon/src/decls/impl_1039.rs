macro_rules! deps {
    () => {
        IntoIter!();
        Iter!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_1039 {
    () => {
        deps!();
        impl < T : Send > IntoParallelIterator for Option < T > { type Item = T ; type Iter = IntoIter < T > ; fn into_par_iter (self) -> Self :: Iter { IntoIter { opt : self } } }
    };
}

impl_1039!()