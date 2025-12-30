// Generated macro for IterableDataProvider (trait)
macro_rules! Depcrate_data_providerIterableDataProvider {
() => {
// Module: crate::data_provider
// Provides: {"IterableDataProvider"}
// Dependencies: {}
# [doc = " A [`DataProvider`] that can iterate over all supported [`DataIdentifierCow`]s."] # [doc = ""] # [doc = " The provider is not allowed to return `Ok` for requests that were not returned by `iter_ids`,"] # [doc = " and must not fail with a [`DataErrorKind::IdentifierNotFound`] for requests that were returned."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] pub trait IterableDataProvider < M : DataMarker > : DataProvider < M > { # [doc = " Returns a set of [`DataIdentifierCow`]."] fn iter_ids (& self) -> Result < alloc :: collections :: BTreeSet < DataIdentifierCow < '_ > > , DataError > ; }
};
}
