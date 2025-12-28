macro_rules! deps {
    () => {
        Iter!();
        IntoIter!();
    };
}

macro_rules! impl_520 {
    () => {
        deps!();
        impl < T , A > fmt :: Debug for IntoIter < T , A > where T : fmt :: Debug , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (Iter { inner : self . inner . iter () , marker : PhantomData , }) . finish () } }
    };
}

impl_520!();