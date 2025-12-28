macro_rules! deps {
    () => {
        IntoParIter!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < T > IntoParallelIterator for Box < Slice < T > > where T : Send , { type Item = T ; type Iter = IntoParIter < T > ; fn into_par_iter (self) -> Self :: Iter { IntoParIter { entries : self . into_entries () , } } }
    };
}

impl_146!()