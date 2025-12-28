macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        impl < T > Clone for Iter < '_ , T > { fn clone (& self) -> Self { Self { rb : self . rb , index : self . index , len : self . len , } } }
    };
}

impl_463!();