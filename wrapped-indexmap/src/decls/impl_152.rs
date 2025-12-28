macro_rules! deps {
    () => {
        ParIter!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < 'a , T > IntoParallelIterator for & 'a Slice < T > where T : Sync , { type Item = & 'a T ; type Iter = ParIter < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { ParIter { entries : & self . entries , } } }
    };
}

impl_152!();