// Generated macro for impl_67 (impl)
macro_rules! Depcrate_exportimpl_67 {
() => {
// Module: crate::export
// Provides: {"impl_67"}
// Dependencies: {}
impl DataExporter for MultiExporter { fn put_payload (& self , marker : DataMarkerInfo , id : DataIdentifierBorrowed , payload : & DataPayload < ExportMarker > ,) -> Result < () , DataError > { self . 0 . iter () . try_for_each (| e | e . put_payload (marker , id , payload)) } fn flush_singleton (& self , marker : DataMarkerInfo , payload : & DataPayload < ExportMarker > , metadata : FlushMetadata ,) -> Result < () , DataError > { self . 0 . iter () . try_for_each (| e | e . flush_singleton (marker , payload , metadata)) } fn flush (& self , marker : DataMarkerInfo , metadata : FlushMetadata) -> Result < () , DataError > { self . 0 . iter () . try_for_each (| e | e . flush (marker , metadata)) } fn close (& mut self) -> Result < ExporterCloseMetadata , DataError > { Ok (ExporterCloseMetadata (Some (Box :: new (self . 0 . iter_mut () . try_fold (vec ! [] , | mut m , e | { m . push (e . close () ? . 0) ; Ok :: < _ , DataError > (m) }) ? ,)))) } }
};
}
