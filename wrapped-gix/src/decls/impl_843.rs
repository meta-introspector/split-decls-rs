macro_rules! deps {
    () => {
        IndexPersistedOrInMemory!();
    };
}

macro_rules! impl_843 {
    () => {
        deps!();
        # [cfg (feature = "index")] impl From < gix_index :: File > for IndexPersistedOrInMemory { fn from (value : gix_index :: File) -> Self { IndexPersistedOrInMemory :: InMemory (value) } }
    };
}

impl_843!();