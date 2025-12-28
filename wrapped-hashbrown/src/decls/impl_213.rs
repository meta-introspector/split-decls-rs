macro_rules! deps {
    () => {
        ParIter!();
        IntoParIter!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for IntoParIter < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ParIter { inner : unsafe { self . inner . par_iter () } , marker : PhantomData , } . fmt (f) } }
    };
}

impl_213!()