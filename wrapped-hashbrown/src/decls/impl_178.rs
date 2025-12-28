macro_rules! deps {
    () => {
        RawParDrain!();
        RawParIter!();
        RawTable!();
        RawIntoParIter!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < T , A : Allocator > RawTable < T , A > { # [doc = " Returns a parallel iterator over the elements in a `RawTable`."] # [cfg_attr (feature = "inline-more" , inline)] pub unsafe fn par_iter (& self) -> RawParIter < T > { RawParIter { iter : self . iter () . iter , } } # [doc = " Returns a parallel iterator over the elements in a `RawTable`."] # [cfg_attr (feature = "inline-more" , inline)] pub fn into_par_iter (self) -> RawIntoParIter < T , A > { RawIntoParIter { table : self } } # [doc = " Returns a parallel iterator which consumes all elements of a `RawTable`"] # [doc = " without freeing its memory allocation."] # [cfg_attr (feature = "inline-more" , inline)] pub fn par_drain (& mut self) -> RawParDrain < '_ , T , A > { RawParDrain { table : NonNull :: from (self) , marker : PhantomData , } } }
    };
}

impl_178!()