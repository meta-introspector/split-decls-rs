macro_rules! deps {
    () => {
        IndexPersistedOrInMemory!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl std :: ops :: Deref for IndexPersistedOrInMemory { type Target = gix_index :: File ; fn deref (& self) -> & Self :: Target { match self { IndexPersistedOrInMemory :: Persisted (i) => i , IndexPersistedOrInMemory :: InMemory (i) => i , } } }
    };
}

impl_341!()