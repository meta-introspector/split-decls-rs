// Generated macro for impl_761 (impl)
macro_rules! Depcrate_extensions_apollo_persisted_queriesimpl_761 {
() => {
// Module: crate::extensions::apollo_persisted_queries
// Provides: {"impl_761"}
// Dependencies: {}
impl < T : CacheStorage > ExtensionFactory for ApolloPersistedQueries < T > { fn create (& self) -> Arc < dyn Extension > { Arc :: new (ApolloPersistedQueriesExtension { storage : self . 0 . clone () , }) } }
};
}
