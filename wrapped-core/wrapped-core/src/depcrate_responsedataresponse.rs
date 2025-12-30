// Generated macro for DataResponse (struct)
macro_rules! Depcrate_responseDataResponse {
() => {
// Module: crate::response
// Provides: {"DataResponse"}
// Dependencies: {}
# [doc = " A response object containing an object as payload and metadata about it."] # [allow (clippy :: exhaustive_structs)] pub struct DataResponse < M > where M : DynamicDataMarker , { # [doc = " Metadata about the returned object."] pub metadata : DataResponseMetadata , # [doc = " The object itself"] pub payload : DataPayload < M > , }
};
}
