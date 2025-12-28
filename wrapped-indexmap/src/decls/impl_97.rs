macro_rules! deps {
    () => {
        IntoParIter!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < K , V > IntoParallelIterator for Box < Slice < K , V > > where K : Send , V : Send , { type Item = (K , V) ; type Iter = IntoParIter < K , V > ; fn into_par_iter (self) -> Self :: Iter { IntoParIter { entries : self . into_entries () , } } }
    };
}

impl_97!()