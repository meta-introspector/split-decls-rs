macro_rules! deps {
    () => {
        Pointable!();
        Shared!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > fmt :: Debug for Shared < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let (raw , tag) = decompose_tag :: < T > (self . data) ; f . debug_struct ("Shared") . field ("raw" , & raw) . field ("tag" , & tag) . finish () } }
    };
}

impl_61!();