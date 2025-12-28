macro_rules! deps {
    () => {
        ParIterMut!();
        IndexMap!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < 'a , K , V , S > IntoParallelIterator for & 'a mut IndexMap < K , V , S > where K : Sync + Send , V : Send , { type Item = (& 'a K , & 'a mut V) ; type Iter = ParIterMut < 'a , K , V > ; fn into_par_iter (self) -> Self :: Iter { ParIterMut { entries : self . as_entries_mut () , } } }
    };
}

impl_109!();