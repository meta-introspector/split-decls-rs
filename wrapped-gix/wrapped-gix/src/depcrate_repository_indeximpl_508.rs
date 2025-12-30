// Generated macro for impl_508 (impl)
macro_rules! Depcrate_repository_indeximpl_508 {
() => {
// Module: crate::repository::index
// Provides: {"impl_508"}
// Dependencies: {}
impl std :: ops :: Deref for IndexPersistedOrInMemory { type Target = gix_index :: File ; fn deref (& self) -> & Self :: Target { match self { IndexPersistedOrInMemory :: Persisted (i) => i , IndexPersistedOrInMemory :: InMemory (i) => i , } } }
};
}
