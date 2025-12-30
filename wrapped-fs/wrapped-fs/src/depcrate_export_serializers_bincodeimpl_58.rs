// Generated macro for impl_58 (impl)
macro_rules! Depcrate_export_serializers_bincodeimpl_58 {
() => {
// Module: crate::export::serializers::bincode
// Provides: {"impl_58"}
// Dependencies: {}
impl AbstractSerializer for Serializer { fn serialize (& self , obj : & DataPayload < ExportMarker > , sink : & mut dyn io :: Write ,) -> Result < () , DataError > { obj . serialize (& mut bincode :: Serializer :: new (sink , bincode :: config :: DefaultOptions :: new () . with_fixint_encoding () ,)) . map_err (| e | DataError :: custom ("Bincode serialize") . with_display_context (& e)) ? ; Ok (()) } fn get_buffer_format (& self) -> BufferFormat { BufferFormat :: Bincode1 } }
};
}
