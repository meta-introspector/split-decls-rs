// Generated macro for DataExporter (trait)
macro_rules! Depcrate_exportDataExporter {
() => {
// Module: crate::export
// Provides: {"DataExporter"}
// Dependencies: {}
# [doc = " An object capable of exporting data payloads in some form."] pub trait DataExporter : Sync { # [doc = " Save a `payload` corresponding to the given marker and locale."] # [doc = ""] # [doc = " Takes non-mut self as it can be called concurrently."] fn put_payload (& self , marker : DataMarkerInfo , id : DataIdentifierBorrowed , payload : & DataPayload < ExportMarker > ,) -> Result < () , DataError > ; # [doc = " Function called for singleton markers."] # [doc = ""] # [doc = " Takes non-mut self as it can be called concurrently."] fn flush_singleton (& self , marker : DataMarkerInfo , payload : & DataPayload < ExportMarker > , metadata : FlushMetadata ,) -> Result < () , DataError > { self . put_payload (marker , Default :: default () , payload) ? ; self . flush (marker , metadata) } # [doc = " Function called after a non-singleton marker has been fully enumerated."] # [doc = ""] # [doc = " Takes non-mut self as it can be called concurrently."] fn flush (& self , _marker : DataMarkerInfo , _metadata : FlushMetadata) -> Result < () , DataError > { Ok (()) } # [doc = " This function has to be called before the object is dropped (after all"] # [doc = " markers have been fully dumped). This conceptually takes ownership, so"] # [doc = " clients *may not* interact with this object after close has been called."] fn close (& mut self) -> Result < ExporterCloseMetadata , DataError > { Ok (ExporterCloseMetadata :: default ()) } }
};
}
