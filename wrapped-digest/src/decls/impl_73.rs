macro_rules! deps {
    () => {
        XofFixedWrapper!();
        ExtendableOutput!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < T : ExtendableOutput + Clone , S : ArraySize > Clone for XofFixedWrapper < T , S > { fn clone (& self) -> Self { Self { hash : self . hash . clone () , size : PhantomData , } } }
    };
}

impl_73!()