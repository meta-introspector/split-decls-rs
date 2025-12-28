macro_rules! deps {
    () => {
        ParIter!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < 'a , K , V > IntoParallelIterator for & 'a Slice < K , V > where K : Sync , V : Sync , { type Item = (& 'a K , & 'a V) ; type Iter = ParIter < 'a , K , V > ; fn into_par_iter (self) -> Self :: Iter { ParIter { entries : & self . entries , } } }
    };
}

impl_103!();