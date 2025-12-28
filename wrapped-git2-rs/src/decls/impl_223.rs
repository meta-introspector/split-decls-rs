macro_rules! deps {
    () => {
        Binding!();
        BlobWriter!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < 'repo > Binding for BlobWriter < 'repo > { type Raw = * mut raw :: git_writestream ; unsafe fn from_raw (raw : * mut raw :: git_writestream) -> BlobWriter < 'repo > { BlobWriter { raw , need_cleanup : true , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_writestream { self . raw } }
    };
}

impl_223!();