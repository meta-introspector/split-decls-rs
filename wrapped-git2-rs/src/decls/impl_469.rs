macro_rules! deps {
    () => {
        Note!();
        Binding!();
    };
}

macro_rules! impl_469 {
    () => {
        deps!();
        impl < 'repo > Binding for Note < 'repo > { type Raw = * mut raw :: git_note ; unsafe fn from_raw (raw : * mut raw :: git_note) -> Note < 'repo > { Note { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_note { self . raw } }
    };
}

impl_469!();