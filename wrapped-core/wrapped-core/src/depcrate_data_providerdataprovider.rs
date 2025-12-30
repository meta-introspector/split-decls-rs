// Generated macro for DataProvider (trait)
macro_rules! Depcrate_data_providerDataProvider {
() => {
// Module: crate::data_provider
// Provides: {"DataProvider"}
// Dependencies: {}
# [doc = " A data provider that loads data for a specific [`DataMarkerInfo`]."] pub trait DataProvider < M > where M : DataMarker , { # [doc = " Query the provider for data, returning the result."] # [doc = ""] # [doc = " Returns [`Ok`] if the request successfully loaded data. If data failed to load, returns an"] # [doc = " Error with more information."] fn load (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > ; }
};
}
