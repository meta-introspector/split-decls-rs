// Generated macro for DataProviderWithMarker (struct)
macro_rules! Depcrate_data_providerDataProviderWithMarker {
() => {
// Module: crate::data_provider
// Provides: {"DataProviderWithMarker"}
// Dependencies: {}
# [doc = " A [`DataProvider`] associated with a specific marker."] # [doc = ""] # [doc = " Implements [`BoundDataProvider`]."] # [derive (Debug)] pub struct DataProviderWithMarker < M , P > { inner : P , _marker : PhantomData < M > , }
};
}
