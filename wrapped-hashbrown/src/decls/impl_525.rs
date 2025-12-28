macro_rules! deps {
    () => {
        Drain!();
        Iter!();
    };
}

macro_rules! impl_525 {
    () => {
        deps!();
        impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for Drain < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (Iter { inner : self . inner . iter () , marker : PhantomData , }) . finish () } }
    };
}

impl_525!();