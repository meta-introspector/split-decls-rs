macro_rules! deps {
    () => {
        IntoParallelIterator!();
        Iter!();
        IntoIter!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < T : Send > IntoParallelIterator for BinaryHeap < T > { type Item = T ; type Iter = IntoIter < T > ; fn into_par_iter (self) -> Self :: Iter { IntoIter { inner : Vec :: from (self) . into_par_iter () , } } }
    };
}

impl_26!();