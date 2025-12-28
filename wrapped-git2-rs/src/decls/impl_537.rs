macro_rules! deps {
    () => {
        PackBuilder!();
        Binding!();
    };
}

macro_rules! impl_537 {
    () => {
        deps!();
        impl < 'repo > Binding for PackBuilder < 'repo > { type Raw = * mut raw :: git_packbuilder ; unsafe fn from_raw (ptr : * mut raw :: git_packbuilder) -> PackBuilder < 'repo > { PackBuilder { raw : ptr , _progress : None , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_packbuilder { self . raw } }
    };
}

impl_537!();