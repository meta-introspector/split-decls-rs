macro_rules! deps {
    () => {
        ServiceFn!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl < F , R > Clone for ServiceFn < F , R > where F : Clone , { fn clone (& self) -> Self { ServiceFn { f : self . f . clone () , _req : PhantomData , } } }
    };
}

impl_204!();