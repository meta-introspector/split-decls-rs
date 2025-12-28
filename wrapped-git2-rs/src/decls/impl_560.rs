macro_rules! deps {
    () => {
        PathspecMatchList!();
        Binding!();
    };
}

macro_rules! impl_560 {
    () => {
        deps!();
        impl < 'ps > Binding for PathspecMatchList < 'ps > { type Raw = * mut raw :: git_pathspec_match_list ; unsafe fn from_raw (raw : * mut raw :: git_pathspec_match_list) -> PathspecMatchList < 'ps > { PathspecMatchList { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_pathspec_match_list { self . raw } }
    };
}

impl_560!();