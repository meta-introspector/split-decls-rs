macro_rules! deps {
    () => {
        HashMap!();
        Iter!();
        ParIterMut!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < 'a , K : Sync , V : Send , S , A : Allocator > IntoParallelIterator for & 'a mut HashMap < K , V , S , A > { type Item = (& 'a K , & 'a mut V) ; type Iter = ParIterMut < 'a , K , V > ; # [cfg_attr (feature = "inline-more" , inline)] fn into_par_iter (self) -> Self :: Iter { ParIterMut { inner : unsafe { self . table . par_iter () } , marker : PhantomData , } } }
    };
}

impl_153!();