macro_rules! deps {
    () => {
        IntoParIter!();
        ParIter!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < K : fmt :: Debug + Eq + Hash , V : fmt :: Debug , A : Allocator > fmt :: Debug for IntoParIter < K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ParIter { inner : unsafe { self . inner . par_iter () } , marker : PhantomData , } . fmt (f) } }
    };
}

impl_144!()