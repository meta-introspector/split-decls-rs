// Generated macro for impl_509 (impl)
macro_rules! Depcrate_repository_indeximpl_509 {
() => {
// Module: crate::repository::index
// Provides: {"impl_509"}
// Dependencies: {}
impl IndexPersistedOrInMemory { # [doc = " Consume this instance and turn it into an owned index file."] # [doc = ""] # [doc = " Note that this will cause the persisted index to be cloned, which would happen whenever the repository has a worktree."] pub fn into_owned (self) -> gix_index :: File { match self { IndexPersistedOrInMemory :: Persisted (i) => gix_index :: File :: clone (& i) , IndexPersistedOrInMemory :: InMemory (i) => i , } } }
};
}
