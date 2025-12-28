macro_rules! deps {
    () => {
        DiffLine!();
        Binding!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl < 'a > Binding for DiffLine < 'a > { type Raw = * const raw :: git_diff_line ; unsafe fn from_raw (raw : * const raw :: git_diff_line) -> DiffLine < 'a > { DiffLine { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_diff_line { self . raw } }
    };
}

impl_345!();