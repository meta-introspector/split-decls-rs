macro_rules! deps {
    () => {
        ParDrain!();
        ParIter!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < K : fmt :: Debug + Eq + Hash , V : fmt :: Debug , A : Allocator > fmt :: Debug for ParDrain < '_ , K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ParIter { inner : unsafe { self . inner . par_iter () } , marker : PhantomData , } . fmt (f) } }
    };
}

impl_147!();