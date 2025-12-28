macro_rules! deps {
    () => {
        IdxRange!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T > Clone for IdxRange < T > { fn clone (& self) -> Self { Self { range : self . range . clone () , _p : PhantomData } } }
    };
}

impl_43!();