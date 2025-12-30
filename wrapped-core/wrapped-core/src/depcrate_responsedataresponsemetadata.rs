// Generated macro for DataResponseMetadata (struct)
macro_rules! Depcrate_responseDataResponseMetadata {
() => {
// Module: crate::response
// Provides: {"DataResponseMetadata"}
// Dependencies: {}
# [doc = " A response object containing metadata about the returned data."] # [derive (Debug , Clone , PartialEq , Default)] # [non_exhaustive] pub struct DataResponseMetadata { # [doc = " The resolved locale of the returned data, if locale fallbacking was performed."] pub locale : Option < DataLocale > , # [doc = " The format of the buffer for buffer-backed data, if known (for example, JSON)."] pub buffer_format : Option < crate :: buf :: BufferFormat > , # [doc = " An optional checksum. This can be used to ensure consistency across different markers."] pub checksum : Option < u64 > , }
};
}
