macro_rules! deps {
    () => {
        ParIter!();
        Iter!();
        HashSet!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl < 'a , T : Sync , S , A : Allocator > IntoParallelIterator for & 'a HashSet < T , S , A > { type Item = & 'a T ; type Iter = ParIter < 'a , T > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { ParIter { inner : self . map . par_keys () , } } }
    };
}

impl_197!();