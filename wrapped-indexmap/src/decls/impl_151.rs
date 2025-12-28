macro_rules! deps {
    () => {
        ParIter!();
        IndexSet!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < 'a , T , S > IntoParallelIterator for & 'a IndexSet < T , S > where T : Sync , { type Item = & 'a T ; type Iter = ParIter < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { ParIter { entries : self . as_entries () , } } }
    };
}

impl_151!();