macro_rules! deps {
    () => {
        Patch!();
        Binding!();
    };
}

macro_rules! impl_546 {
    () => {
        deps!();
        impl < 'buffers > Binding for Patch < 'buffers > { type Raw = * mut raw :: git_patch ; unsafe fn from_raw (raw : Self :: Raw) -> Self { Patch { raw , buffers : PhantomData , } } fn raw (& self) -> Self :: Raw { self . raw } }
    };
}

impl_546!()