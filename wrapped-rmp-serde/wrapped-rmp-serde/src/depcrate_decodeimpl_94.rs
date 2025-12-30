// Generated macro for impl_94 (impl)
macro_rules! Depcrate_decodeimpl_94 {
() => {
// Module: crate::decode
// Provides: {"impl_94"}
// Dependencies: {}
# [doc = " Deserializer for Ext `SeqAccess`"] impl < 'de , 'a , R : ReadSlice < 'de > + 'a , C : SerializerConfig > de :: Deserializer < 'de > for & mut ExtDeserializer < 'a , R , C > { type Error = Error ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > { match self . state { ExtDeserializerState :: New => { let tag = self . rd . read_data_i8 () ? ; self . state = ExtDeserializerState :: ReadTag ; visitor . visit_i8 (tag) } ExtDeserializerState :: ReadTag => { let data = self . rd . read_slice (self . len as usize) . map_err (Error :: InvalidDataRead) ? ; self . state = ExtDeserializerState :: ReadBinary ; match data { Reference :: Borrowed (bytes) => visitor . visit_borrowed_bytes (bytes) , Reference :: Copied (bytes) => visitor . visit_bytes (bytes) , } } ExtDeserializerState :: ReadBinary => { debug_assert ! (false) ; Err (Error :: TypeMismatch (Marker :: Reserved)) } , } } forward_to_deserialize_any ! { bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option seq bytes byte_buf map unit_struct newtype_struct tuple_struct struct identifier tuple enum ignored_any } }
};
}
