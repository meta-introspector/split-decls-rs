macro_rules! deps {
    () => {
        Binding!();
        Object!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        impl < 'repo > Binding for Object < 'repo > { type Raw = * mut raw :: git_object ; unsafe fn from_raw (raw : * mut raw :: git_object) -> Object < 'repo > { Object { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_object { self . raw } }
    };
}

impl_483!()