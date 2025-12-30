// Generated macro for deserialize_string_record (function)
macro_rules! Depcrate_deserializerdeserialize_string_record {
() => {
// Module: crate::deserializer
// Provides: {"deserialize_string_record"}
// Dependencies: {}
pub fn deserialize_string_record < 'de , D : Deserialize < 'de > > (record : & 'de StringRecord , headers : Option < & 'de StringRecord > ,) -> Result < D , Error > { let mut deser = DeRecordWrap (DeStringRecord { it : record . iter () . peekable () , headers : headers . map (| r | r . iter ()) , field : 0 , }) ; D :: deserialize (& mut deser) . map_err (| err | { Error :: new (ErrorKind :: Deserialize { pos : record . position () . cloned () , err , }) }) }
};
}
