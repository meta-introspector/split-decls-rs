// Generated macro for impl_59 (impl)
macro_rules! Depcrate_exportimpl_59 {
() => {
// Module: crate::export
// Provides: {"impl_59"}
// Dependencies: {}
impl DataExporter for Box < dyn DataExporter > { fn put_payload (& self , marker : DataMarkerInfo , id : DataIdentifierBorrowed , payload : & DataPayload < ExportMarker > ,) -> Result < () , DataError > { (* * self) . put_payload (marker , id , payload) } fn flush_singleton (& self , marker : DataMarkerInfo , payload : & DataPayload < ExportMarker > , metadata : FlushMetadata ,) -> Result < () , DataError > { (* * self) . flush_singleton (marker , payload , metadata) } fn flush (& self , marker : DataMarkerInfo , metadata : FlushMetadata) -> Result < () , DataError > { (* * self) . flush (marker , metadata) } fn close (& mut self) -> Result < ExporterCloseMetadata , DataError > { (* * self) . close () } }
};
}
