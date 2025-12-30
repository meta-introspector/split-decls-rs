// Generated macro for impl_71 (impl)
macro_rules! Depcrate_export_serializers_jsonimpl_71 {
() => {
// Module: crate::export::serializers::json
// Provides: {"impl_71"}
// Dependencies: {}
impl AbstractSerializer for Serializer { fn serialize (& self , obj : & DataPayload < ExportMarker > , mut sink : & mut dyn io :: Write ,) -> Result < () , DataError > { match self . style { StyleOption :: Compact => obj . serialize (& mut serde_json :: Serializer :: new (& mut sink)) , StyleOption :: Pretty => obj . serialize (& mut serde_json :: Serializer :: pretty (& mut sink)) , } . map_err (| e | DataError :: custom ("JSON serialize") . with_display_context (& e)) ? ; writeln ! (sink) ? ; Ok (()) } fn get_buffer_format (& self) -> BufferFormat { BufferFormat :: Json } fn is_text_format (& self) -> bool { true } }
};
}
