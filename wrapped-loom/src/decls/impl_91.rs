macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < T > Clone for Ref < T > { fn clone (& self) -> Ref < T > { Ref { index : self . index , _p : PhantomData , } } }
    };
}

impl_91!();