macro_rules! deps {
    () => {
        ExtendableOutput!();
        XofFixedWrapper!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < T : ExtendableOutput + KeyInit , S : ArraySize > KeyInit for XofFixedWrapper < T , S > { fn new (key : & crypto_common :: Key < Self >) -> Self { Self { hash : T :: new (key) , size : PhantomData , } } }
    };
}

impl_81!();