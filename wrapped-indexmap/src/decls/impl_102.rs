macro_rules! deps {
    () => {
        IndexMap!();
        ParIter!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < 'a , K , V , S > IntoParallelIterator for & 'a IndexMap < K , V , S > where K : Sync , V : Sync , { type Item = (& 'a K , & 'a V) ; type Iter = ParIter < 'a , K , V > ; fn into_par_iter (self) -> Self :: Iter { ParIter { entries : self . as_entries () , } } }
    };
}

impl_102!();