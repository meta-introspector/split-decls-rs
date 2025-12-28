macro_rules! deps {
    () => {
        Normalized!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl < 'tcx , T > Normalized < 'tcx , T > { pub fn with < U > (self , value : U) -> Normalized < 'tcx , U > { Normalized { value , obligations : self . obligations } } }
    };
}

impl_290!();