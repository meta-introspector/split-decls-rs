// Generated macro for DynamicDryDataProvider (trait)
macro_rules! Depcrate_data_providerDynamicDryDataProvider {
() => {
// Module: crate::data_provider
// Provides: {"DynamicDryDataProvider"}
// Dependencies: {}
# [doc = " A dynanmic data provider that can determine whether it can load a particular data identifier,"] # [doc = " potentially cheaper than actually performing the load."] pub trait DynamicDryDataProvider < M : DynamicDataMarker > : DynamicDataProvider < M > { # [doc = " This method goes through the motions of [`load_data`], but only returns the metadata."] # [doc = ""] # [doc = " If `dry_load_data` returns an error, [`load_data`] must return the same error, but"] # [doc = " not vice-versa. Concretely, [`load_data`] could return deserialization or I/O errors"] # [doc = " that `dry_load_data` cannot predict."] # [doc = ""] # [doc = " [`load_data`]: DynamicDataProvider::load_data"] fn dry_load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponseMetadata , DataError > ; }
};
}
