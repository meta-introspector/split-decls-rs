// Generated macro for DynamicDataProvider (trait)
macro_rules! Depcrate_data_providerDynamicDataProvider {
() => {
// Module: crate::data_provider
// Provides: {"DynamicDataProvider"}
// Dependencies: {}
# [doc = " A data provider that loads data for a specific data type."] # [doc = ""] # [doc = " Unlike [`DataProvider`], there may be multiple markers corresponding to the same data type."] pub trait DynamicDataProvider < M > where M : DynamicDataMarker , { # [doc = " Query the provider for data, returning the result."] # [doc = ""] # [doc = " Returns [`Ok`] if the request successfully loaded data. If data failed to load, returns an"] # [doc = " Error with more information."] fn load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponse < M > , DataError > ; }
};
}
