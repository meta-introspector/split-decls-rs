// Generated macro for IterableDynamicDataProvider (trait)
macro_rules! Depcrate_data_providerIterableDynamicDataProvider {
() => {
// Module: crate::data_provider
// Provides: {"IterableDynamicDataProvider"}
// Dependencies: {}
# [doc = " A [`DynamicDataProvider`] that can iterate over all supported [`DataIdentifierCow`]s for a certain marker."] # [doc = ""] # [doc = " The provider is not allowed to return `Ok` for requests that were not returned by `iter_ids`,"] # [doc = " and must not fail with a [`DataErrorKind::IdentifierNotFound`] for requests that were returned."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] pub trait IterableDynamicDataProvider < M : DynamicDataMarker > : DynamicDataProvider < M > { # [doc = " Given a [`DataMarkerInfo`], returns a set of [`DataIdentifierCow`]."] fn iter_ids_for_marker (& self , marker : DataMarkerInfo ,) -> Result < alloc :: collections :: BTreeSet < DataIdentifierCow < '_ > > , DataError > ; }
};
}
