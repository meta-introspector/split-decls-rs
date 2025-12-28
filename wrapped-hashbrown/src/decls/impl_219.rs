macro_rules! deps {
    () => {
        Iter!();
        ParIter!();
        HashTable!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl < 'a , T : Sync , A : Allocator > IntoParallelIterator for & 'a HashTable < T , A > { type Item = & 'a T ; type Iter = ParIter < 'a , T > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { ParIter { inner : unsafe { self . raw . par_iter () } , marker : PhantomData , } } }
    };
}

impl_219!()