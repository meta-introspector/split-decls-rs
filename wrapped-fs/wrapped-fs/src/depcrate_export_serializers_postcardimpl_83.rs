// Generated macro for impl_83 (impl)
macro_rules! Depcrate_export_serializers_postcardimpl_83 {
() => {
// Module: crate::export::serializers::postcard
// Provides: {"impl_83"}
// Dependencies: {}
impl AbstractSerializer for Serializer { fn serialize (& self , obj : & DataPayload < ExportMarker > , sink : & mut dyn io :: Write ,) -> Result < () , DataError > { let mut serializer = postcard :: Serializer { output : AllocVec :: new () , } ; obj . serialize (& mut serializer) . map_err (| e | DataError :: custom ("Postcard serialize") . with_display_context (& e)) ? ; let output = serializer . output . finalize () . map_err (| _ | DataError :: custom ("Postcard finalize")) ? ; sink . write_all (& output) ? ; Ok (()) } fn get_buffer_format (& self) -> BufferFormat { BufferFormat :: Postcard1 } }
};
}
