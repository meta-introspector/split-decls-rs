// Generated macro for AbstractSerializer (trait)
macro_rules! Depcrate_export_serializersAbstractSerializer {
() => {
// Module: crate::export::serializers
// Provides: {"AbstractSerializer"}
// Dependencies: {}
# [doc = " A simple serializer trait that works on whole objects."] # [doc = ""] # [doc = " This trait is not meant to be implemented by clients."] pub trait AbstractSerializer : core :: fmt :: Debug + seal :: Sealed { # [doc = " Serializes an object to a sink."] # [doc (hidden)] fn serialize (& self , obj : & DataPayload < ExportMarker > , sink : & mut dyn io :: Write ,) -> Result < () , DataError > ; # [doc = " Gets the buffer format currently being serialized."] # [doc (hidden)] fn get_buffer_format (& self) -> BufferFormat ; # [doc = " This can be set to get correct CRLF on Windows."] # [doc (hidden)] fn is_text_format (& self) -> bool { false } }
};
}
