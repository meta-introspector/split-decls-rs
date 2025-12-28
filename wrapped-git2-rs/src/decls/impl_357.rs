macro_rules! deps {
    () => {
        DiffBinaryFile!();
        Binding!();
    };
}

macro_rules! impl_357 {
    () => {
        deps!();
        impl < 'a > Binding for DiffBinaryFile < 'a > { type Raw = * const raw :: git_diff_binary_file ; unsafe fn from_raw (raw : * const raw :: git_diff_binary_file) -> DiffBinaryFile < 'a > { DiffBinaryFile { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_diff_binary_file { self . raw } }
    };
}

impl_357!()