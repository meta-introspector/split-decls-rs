// Generated macro for impl_49 (impl)
macro_rules! Depcrate_export_blob_exporterimpl_49 {
() => {
// Module: crate::export::blob_exporter
// Provides: {"impl_49"}
// Dependencies: {}
impl DataExporter for BlobExporter < '_ > { fn put_payload (& self , marker : DataMarkerInfo , id : DataIdentifierBorrowed , payload : & DataPayload < ExportMarker > ,) -> Result < () , DataError > { let mut serializer = postcard :: Serializer { output : AllocVec :: new () , } ; payload . serialize (& mut serializer) ? ; let output = serializer . output . finalize () . expect ("Failed to finalize serializer output") ; let idx = { let mut unique_resources = self . unique_resources . lock () . expect ("poison") ; let len = unique_resources . len () ; * unique_resources . entry (output) . or_insert (len) } ; # [expect (clippy :: expect_used)] self . resources . lock () . expect ("poison") . entry (marker . id . hashed ()) . or_default () . entry ({ let mut key = id . locale . to_string () ; if ! id . marker_attributes . is_empty () { key . push (crate :: blob_schema :: REQUEST_SEPARATOR) ; key . push_str (id . marker_attributes) ; } key . into_bytes () }) . or_insert (idx) ; Ok (()) } fn flush (& self , marker : DataMarkerInfo , metadata : FlushMetadata) -> Result < () , DataError > { if let Some (checksum) = metadata . checksum { self . checksums . lock () . expect ("poison") . insert (marker . id . hashed () , checksum) ; } self . all_markers . lock () . expect ("poison") . insert (marker . id . hashed ()) ; Ok (()) } fn close (& mut self) -> Result < ExporterCloseMetadata , DataError > { self . close_internal () } }
};
}
