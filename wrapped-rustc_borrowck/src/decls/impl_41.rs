macro_rules! deps {
    () => {
        OutlivesConstraint!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < 'tcx > fmt :: Debug for OutlivesConstraint < 'tcx > { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (formatter , "({:?}: {:?}) due to {:?} ({:?}) ({:?})" , self . sup , self . sub , self . locations , self . variance_info , self . category ,) } }
    };
}

impl_41!();