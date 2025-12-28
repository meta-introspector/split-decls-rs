macro_rules! deps {
    () => {
        IntoParIter!();
        HashSet!();
        Iter!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl < T : Send , S , A : Allocator + Send > IntoParallelIterator for HashSet < T , S , A > { type Item = T ; type Iter = IntoParIter < T , A > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { IntoParIter { inner : self . map . into_par_iter () , } } }
    };
}

impl_196!()