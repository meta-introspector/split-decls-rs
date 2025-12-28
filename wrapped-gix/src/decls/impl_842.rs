macro_rules! deps {
    () => {
        IndexPersistedOrInMemory!();
        Index!();
    };
}

macro_rules! impl_842 {
    () => {
        deps!();
        # [cfg (feature = "index")] impl From < Index > for IndexPersistedOrInMemory { fn from (value : Index) -> Self { IndexPersistedOrInMemory :: Persisted (value) } }
    };
}

impl_842!()