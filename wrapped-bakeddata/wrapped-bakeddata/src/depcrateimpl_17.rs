// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl < E : DataExporter > DataExporter for StubExporter < E > { fn put_payload (& self , marker : DataMarkerInfo , id : DataIdentifierBorrowed , payload : & DataPayload < ExportMarker > ,) -> Result < () , DataError > { if id . locale . is_unknown () && marker . expose_baked_consts { self . 0 . put_payload (marker , id , payload) } else { Ok (()) } } fn flush_singleton (& self , marker : DataMarkerInfo , payload : & DataPayload < ExportMarker > , metadata : FlushMetadata ,) -> Result < () , DataError > { self . 0 . flush_singleton (marker , payload , metadata) } fn flush (& self , marker : DataMarkerInfo , metadata : FlushMetadata) -> Result < () , DataError > { self . 0 . flush (marker , metadata) } fn close (& mut self) -> Result < ExporterCloseMetadata , DataError > { self . 0 . close () } }
};
}
