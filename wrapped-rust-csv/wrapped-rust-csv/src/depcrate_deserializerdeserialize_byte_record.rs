// Generated macro for deserialize_byte_record (function)
macro_rules! Depcrate_deserializerdeserialize_byte_record {
() => {
// Module: crate::deserializer
// Provides: {"deserialize_byte_record"}
// Dependencies: {}
pub fn deserialize_byte_record < 'de , D : Deserialize < 'de > > (record : & 'de ByteRecord , headers : Option < & 'de ByteRecord > ,) -> Result < D , Error > { let mut deser = DeRecordWrap (DeByteRecord { it : record . iter () . peekable () , headers : headers . map (| r | r . iter ()) , field : 0 , }) ; D :: deserialize (& mut deser) . map_err (| err | { Error :: new (ErrorKind :: Deserialize { pos : record . position () . cloned () , err , }) }) }
};
}
