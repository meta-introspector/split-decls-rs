// Generated macro for impl_48 (impl)
macro_rules! Depcrate_export_blob_exporterimpl_48 {
() => {
// Module: crate::export::blob_exporter
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'w > BlobExporter < 'w > { # [doc = " Creates a version 1 [`BlobExporter`] that writes to the given I/O stream."] # [doc = ""] # [doc = " Version 1 is needed if the blob may be consumed by ICU4X versions 1.0 through 1.3. If"] # [doc = " targeting only ICU4X 1.4 and above, see [BlobExporter::new_with_sink()]."] pub fn new_with_sink (sink : Box < dyn std :: io :: Write + Sync + 'w >) -> Self { Self { resources : Default :: default () , unique_resources : Default :: default () , checksums : Default :: default () , all_markers : Default :: default () , sink , } } }
};
}
