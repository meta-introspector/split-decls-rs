macro_rules! deps {
    () => {
        Binding!();
        AnnotatedCommit!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl < 'repo > Binding for AnnotatedCommit < 'repo > { type Raw = * mut raw :: git_annotated_commit ; unsafe fn from_raw (raw : * mut raw :: git_annotated_commit) -> AnnotatedCommit < 'repo > { AnnotatedCommit { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_annotated_commit { self . raw } }
    };
}

impl_427!();