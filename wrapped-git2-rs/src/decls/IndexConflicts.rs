macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! IndexConflicts {
    () => {
        deps!();
        # [doc = " An iterator over the conflicting entries in an index"] pub struct IndexConflicts < 'index > { conflict_iter : * mut raw :: git_index_conflict_iterator , _marker : marker :: PhantomData < & 'index Index > , }
    };
}

IndexConflicts!();