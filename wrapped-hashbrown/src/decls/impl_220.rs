macro_rules! deps {
    () => {
        ParIterMut!();
        HashTable!();
        Iter!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl < 'a , T : Send , A : Allocator > IntoParallelIterator for & 'a mut HashTable < T , A > { type Item = & 'a mut T ; type Iter = ParIterMut < 'a , T > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { ParIterMut { inner : unsafe { self . raw . par_iter () } , marker : PhantomData , } } }
    };
}

impl_220!();