macro_rules! deps {
    () => {
        Note!();
        IndexPersistedOrInMemory!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl IndexPersistedOrInMemory { # [doc = " Consume this instance and turn it into an owned index file."] # [doc = ""] # [doc = " Note that this will cause the persisted index to be cloned, which would happen whenever the repository has a worktree."] pub fn into_owned (self) -> gix_index :: File { match self { IndexPersistedOrInMemory :: Persisted (i) => gix_index :: File :: clone (& i) , IndexPersistedOrInMemory :: InMemory (i) => i , } } }
    };
}

impl_342!()