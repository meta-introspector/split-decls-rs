// Generated macro for FlushMetadata (struct)
macro_rules! Depcrate_exportFlushMetadata {
() => {
// Module: crate::export
// Provides: {"FlushMetadata"}
// Dependencies: {}
# [doc = " Metadata for [`DataExporter::flush`]"] # [non_exhaustive] # [derive (Debug , Copy , Clone , Default)] pub struct FlushMetadata { # [doc = " Whether the data was generated in such a way that a [`DryDataProvider`] implementation"] # [doc = " makes sense."] pub supports_dry_provider : bool , # [doc = " The checksum to return with this data marker."] pub checksum : Option < u64 > , }
};
}
