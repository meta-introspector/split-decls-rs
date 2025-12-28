macro_rules! deps {
    () => {
        Binding!();
        IndexConflicts!();
    };
}

macro_rules! impl_389 {
    () => {
        deps!();
        impl < 'index > Binding for IndexConflicts < 'index > { type Raw = * mut raw :: git_index_conflict_iterator ; unsafe fn from_raw (raw : * mut raw :: git_index_conflict_iterator) -> IndexConflicts < 'index > { IndexConflicts { conflict_iter : raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_index_conflict_iterator { self . conflict_iter } }
    };
}

impl_389!()