macro_rules! deps {
    () => {
        MismatchedProjectionTypes!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl < 'tcx > fmt :: Debug for traits :: MismatchedProjectionTypes < 'tcx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "MismatchedProjectionTypes({:?})" , self . err) } }
    };
}

impl_302!()