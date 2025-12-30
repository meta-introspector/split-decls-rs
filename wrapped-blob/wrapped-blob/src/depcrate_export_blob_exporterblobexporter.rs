// Generated macro for BlobExporter (struct)
macro_rules! Depcrate_export_blob_exporterBlobExporter {
() => {
// Module: crate::export::blob_exporter
// Provides: {"BlobExporter"}
// Dependencies: {}
# [doc = " A data exporter that writes data to a single-file blob."] # [doc = " See the module-level docs for an example."] pub struct BlobExporter < 'w > { # [doc = " Map of marker path hash -> locale byte string -> blob ID"] resources : Mutex < BTreeMap < DataMarkerIdHash , BTreeMap < Vec < u8 > , usize > > > , checksums : Mutex < BTreeMap < DataMarkerIdHash , u64 > > , all_markers : Mutex < BTreeSet < DataMarkerIdHash > > , # [doc = " Map from blob to blob ID"] unique_resources : Mutex < HashMap < Vec < u8 > , usize > > , sink : Box < dyn std :: io :: Write + Sync + 'w > , }
};
}
