// Generated macro for BoundDataProvider (trait)
macro_rules! Depcrate_data_providerBoundDataProvider {
() => {
// Module: crate::data_provider
// Provides: {"BoundDataProvider"}
// Dependencies: {}
# [doc = " A data provider that loads data for a specific data type."] # [doc = ""] # [doc = " Unlike [`DataProvider`], the provider is bound to a specific marker ahead of time."] # [doc = ""] # [doc = " This crate provides [`DataProviderWithMarker`] which implements this trait on a single provider"] # [doc = " with a single marker. However, this trait can also be implemented on providers that fork between"] # [doc = " multiple markers that all return the same data type. For example, it can abstract over many"] # [doc = " calendar systems in the datetime formatter."] pub trait BoundDataProvider < M > where M : DynamicDataMarker , { # [doc = " Query the provider for data, returning the result."] # [doc = ""] # [doc = " Returns [`Ok`] if the request successfully loaded data. If data failed to load, returns an"] # [doc = " Error with more information."] fn load_bound (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > ; # [doc = " Returns the [`DataMarkerInfo`] that this provider uses for loading data."] fn bound_marker (& self) -> DataMarkerInfo ; }
};
}
