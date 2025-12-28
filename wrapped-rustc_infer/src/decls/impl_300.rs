macro_rules! deps {
    () => {
        Normalized!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        impl < 'tcx , T : fmt :: Debug > fmt :: Debug for Normalized < 'tcx , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Normalized({:?}, {:?})" , self . value , self . obligations) } }
    };
}

impl_300!()