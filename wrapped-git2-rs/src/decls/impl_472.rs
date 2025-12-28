macro_rules! deps {
    () => {
        Notes!();
        Binding!();
    };
}

macro_rules! impl_472 {
    () => {
        deps!();
        impl < 'repo > Binding for Notes < 'repo > { type Raw = * mut raw :: git_note_iterator ; unsafe fn from_raw (raw : * mut raw :: git_note_iterator) -> Notes < 'repo > { Notes { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_note_iterator { self . raw } }
    };
}

impl_472!()