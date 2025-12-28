macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_394 {
    () => {
        deps!();
        impl < T > Clone for Empty < T > { fn clone (& self) -> Self { Empty { marker : PhantomData , } } }
    };
}

impl_394!();