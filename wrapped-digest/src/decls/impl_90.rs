macro_rules! deps {
    () => {
        XofFixedWrapper!();
        CustomizedInit!();
        ExtendableOutput!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < T : ExtendableOutput + CustomizedInit , S : ArraySize > CustomizedInit for XofFixedWrapper < T , S > { fn new_customized (customization : & [u8]) -> Self { Self { hash : T :: new_customized (customization) , size : PhantomData , } } }
    };
}

impl_90!()