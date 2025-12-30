// Generated macro for DataRequest (struct)
macro_rules! Depcrate_requestDataRequest {
() => {
// Module: crate::request
// Provides: {"DataRequest"}
// Dependencies: {}
# [doc = " The request type passed into all data provider implementations."] # [derive (Default , Debug , Clone , Copy , PartialEq , Eq)] # [allow (clippy :: exhaustive_structs)] pub struct DataRequest < 'a > { # [doc = " The data identifier for which to load data."] # [doc = ""] # [doc = " If locale fallback is enabled, the resulting data may be from a different identifier"] # [doc = " than the one requested here."] pub id : DataIdentifierBorrowed < 'a > , # [doc = " Metadata that may affect the behavior of the data provider."] pub metadata : DataRequestMetadata , }
};
}
