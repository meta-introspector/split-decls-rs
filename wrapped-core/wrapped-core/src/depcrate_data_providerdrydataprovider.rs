// Generated macro for DryDataProvider (trait)
macro_rules! Depcrate_data_providerDryDataProvider {
() => {
// Module: crate::data_provider
// Provides: {"DryDataProvider"}
// Dependencies: {}
# [doc = " A data provider that can determine whether it can load a particular data identifier,"] # [doc = " potentially cheaper than actually performing the load."] pub trait DryDataProvider < M : DataMarker > : DataProvider < M > { # [doc = " This method goes through the motions of [`load`], but only returns the metadata."] # [doc = ""] # [doc = " If `dry_load` returns an error, [`load`] must return the same error, but"] # [doc = " not vice-versa. Concretely, [`load`] could return deserialization or I/O errors"] # [doc = " that `dry_load` cannot predict."] # [doc = ""] # [doc = " [`load`]: DataProvider::load"] fn dry_load (& self , req : DataRequest) -> Result < DataResponseMetadata , DataError > ; }
};
}
