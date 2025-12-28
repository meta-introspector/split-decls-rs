macro_rules! deps {
    () => {
        IndexSet!();
        IntoParIter!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < T , S > IntoParallelIterator for IndexSet < T , S > where T : Send , { type Item = T ; type Iter = IntoParIter < T > ; fn into_par_iter (self) -> Self :: Iter { IntoParIter { entries : self . into_entries () , } } }
    };
}

impl_145!()