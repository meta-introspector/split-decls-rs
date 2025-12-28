macro_rules! deps {
    () => {
        BlameHunk!();
        Binding!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl < 'blame > Binding for BlameHunk < 'blame > { type Raw = * mut raw :: git_blame_hunk ; unsafe fn from_raw (raw : * mut raw :: git_blame_hunk) -> BlameHunk < 'blame > { BlameHunk { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_blame_hunk { self . raw } }
    };
}

impl_207!();