macro_rules! deps {
    () => {
        Binding!();
        DiffBinary!();
    };
}

macro_rules! impl_355 {
    () => {
        deps!();
        impl < 'a > Binding for DiffBinary < 'a > { type Raw = * const raw :: git_diff_binary ; unsafe fn from_raw (raw : * const raw :: git_diff_binary) -> DiffBinary < 'a > { DiffBinary { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_diff_binary { self . raw } }
    };
}

impl_355!();