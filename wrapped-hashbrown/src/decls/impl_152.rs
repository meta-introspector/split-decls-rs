macro_rules! deps {
    () => {
        HashMap!();
        ParIter!();
        Iter!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < 'a , K : Sync , V : Sync , S , A : Allocator > IntoParallelIterator for & 'a HashMap < K , V , S , A > { type Item = (& 'a K , & 'a V) ; type Iter = ParIter < 'a , K , V > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { ParIter { inner : unsafe { self . table . par_iter () } , marker : PhantomData , } } }
    };
}

impl_152!();