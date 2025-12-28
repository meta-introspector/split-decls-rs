macro_rules! deps {
    () => {
        DiffFile!();
        Binding!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl < 'a > Binding for DiffFile < 'a > { type Raw = * const raw :: git_diff_file ; unsafe fn from_raw (raw : * const raw :: git_diff_file) -> DiffFile < 'a > { DiffFile { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_diff_file { self . raw } }
    };
}

impl_334!()