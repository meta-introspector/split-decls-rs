macro_rules! deps {
    () => {
        HashTable!();
        Iter!();
        IntoParIter!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl < T : Send , A : Allocator + Send > IntoParallelIterator for HashTable < T , A > { type Item = T ; type Iter = IntoParIter < T , A > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { IntoParIter { inner : self . raw . into_par_iter () , } } }
    };
}

impl_218!()