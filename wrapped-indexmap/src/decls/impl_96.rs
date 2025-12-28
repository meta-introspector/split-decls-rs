macro_rules! deps {
    () => {
        IntoParIter!();
        IndexMap!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < K , V , S > IntoParallelIterator for IndexMap < K , V , S > where K : Send , V : Send , { type Item = (K , V) ; type Iter = IntoParIter < K , V > ; fn into_par_iter (self) -> Self :: Iter { IntoParIter { entries : self . into_entries () , } } }
    };
}

impl_96!()