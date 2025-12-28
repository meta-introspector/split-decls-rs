macro_rules! deps {
    () => {
        Signature!();
        Binding!();
    };
}

macro_rules! impl_722 {
    () => {
        deps!();
        impl < 'a > Binding for Signature < 'a > { type Raw = * mut raw :: git_signature ; unsafe fn from_raw (raw : * mut raw :: git_signature) -> Signature < 'a > { Signature { raw , _marker : marker :: PhantomData , owned : true , } } fn raw (& self) -> * mut raw :: git_signature { self . raw } }
    };
}

impl_722!();