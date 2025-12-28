macro_rules! deps {
    () => {
        GenericKind!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < 'tcx > fmt :: Display for GenericKind < 'tcx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { GenericKind :: Param (ref p) => write ! (f , "{p}") , GenericKind :: Placeholder (ref p) => write ! (f , "{p}") , GenericKind :: Alias (ref p) => write ! (f , "{p}") , } } }
    };
}

impl_143!();