macro_rules! deps {
    () => {
        Iter!();
        IntoParIter!();
        HashMap!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl < K : Send , V : Send , S , A : Allocator + Send > IntoParallelIterator for HashMap < K , V , S , A > { type Item = (K , V) ; type Iter = IntoParIter < K , V , A > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { IntoParIter { inner : self . table . into_par_iter () , } } }
    };
}

impl_151!()