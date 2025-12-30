// Generated macro for DataRequestMetadata (struct)
macro_rules! Depcrate_requestDataRequestMetadata {
() => {
// Module: crate::request
// Provides: {"DataRequestMetadata"}
// Dependencies: {}
# [doc = " Metadata for data requests. This is currently empty, but it may be extended with options"] # [doc = " for tuning locale fallback, buffer layout, and so forth."] # [derive (Default , Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] # [non_exhaustive] pub struct DataRequestMetadata { # [doc = " Silent requests do not log errors. This can be used for exploratory querying, such as fallbacks."] pub silent : bool , # [doc = " Whether to allow prefix matches for the data marker attributes."] pub attributes_prefix_match : bool , }
};
}
