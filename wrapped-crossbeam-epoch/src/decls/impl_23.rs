macro_rules! deps {
    () => {
        Atomic!();
        Pointable!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > fmt :: Debug for Atomic < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let data = self . data . load (Ordering :: SeqCst) ; let (raw , tag) = decompose_tag :: < T > (data) ; f . debug_struct ("Atomic") . field ("raw" , & raw) . field ("tag" , & tag) . finish () } }
    };
}

impl_23!();