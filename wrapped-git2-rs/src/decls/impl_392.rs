macro_rules! deps {
    () => {
        IndexConflicts!();
    };
}

macro_rules! impl_392 {
    () => {
        deps!();
        impl < 'index > Drop for IndexConflicts < 'index > { fn drop (& mut self) { unsafe { raw :: git_index_conflict_iterator_free (self . conflict_iter) } } }
    };
}

impl_392!()