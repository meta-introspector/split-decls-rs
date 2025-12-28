macro_rules! deps {
    () => {
        ParDrain!();
        ParIter!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for ParDrain < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ParIter { inner : unsafe { self . inner . par_iter () } , marker : PhantomData , } . fmt (f) } }
    };
}

impl_216!();