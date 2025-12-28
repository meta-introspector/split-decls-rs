macro_rules! deps {
    () => {
        ParIterMut!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < 'a , K , V > IntoParallelIterator for & 'a mut Slice < K , V > where K : Sync + Send , V : Send , { type Item = (& 'a K , & 'a mut V) ; type Iter = ParIterMut < 'a , K , V > ; fn into_par_iter (self) -> Self :: Iter { ParIterMut { entries : & mut self . entries , } } }
    };
}

impl_110!();