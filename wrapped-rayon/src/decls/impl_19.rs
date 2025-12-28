macro_rules! deps {
    () => {
        IntoParallelIterator!();
        Iter!();
        IntoIter!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < T : Send , const N : usize > IntoParallelIterator for [T ; N] { type Item = T ; type Iter = IntoIter < T , N > ; fn into_par_iter (self) -> Self :: Iter { IntoIter { array : self } } }
    };
}

impl_19!();