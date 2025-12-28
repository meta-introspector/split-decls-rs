macro_rules! deps {
    () => {
        DiffHunk!();
        Binding!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl < 'a > Binding for DiffHunk < 'a > { type Raw = * const raw :: git_diff_hunk ; unsafe fn from_raw (raw : * const raw :: git_diff_hunk) -> DiffHunk < 'a > { DiffHunk { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_diff_hunk { self . raw } }
    };
}

impl_348!();