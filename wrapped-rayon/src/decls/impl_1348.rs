macro_rules! deps {
    () => {
        IntoIter!();
        Iter!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_1348 {
    () => {
        deps!();
        impl < T : Send > IntoParallelIterator for Box < [T] > { type Item = T ; type Iter = IntoIter < T > ; fn into_par_iter (self) -> Self :: Iter { IntoIter { vec : self . into () } } }
    };
}

impl_1348!()