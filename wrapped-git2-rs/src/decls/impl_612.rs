macro_rules! deps {
    () => {
        Binding!();
        References!();
    };
}

macro_rules! impl_612 {
    () => {
        deps!();
        impl < 'repo > Binding for References < 'repo > { type Raw = * mut raw :: git_reference_iterator ; unsafe fn from_raw (raw : * mut raw :: git_reference_iterator) -> References < 'repo > { References { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_reference_iterator { self . raw } }
    };
}

impl_612!()