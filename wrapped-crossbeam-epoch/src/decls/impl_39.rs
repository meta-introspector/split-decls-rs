macro_rules! deps {
    () => {
        Pointable!();
        Owned!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > fmt :: Debug for Owned < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let (raw , tag) = decompose_tag :: < T > (self . data) ; f . debug_struct ("Owned") . field ("raw" , & raw) . field ("tag" , & tag) . finish () } }
    };
}

impl_39!()