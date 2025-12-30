// Generated macro for impl_92 (impl)
macro_rules! Depcrate_decodeimpl_92 {
() => {
// Module: crate::decode
// Provides: {"impl_92"}
// Dependencies: {}
impl < 'de , 'a , R : ReadSlice < 'de > + 'a , C : SerializerConfig > de :: Deserializer < 'de > for ExtDeserializer < 'a , R , C > { type Error = Error ; # [inline (always)] fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > { visitor . visit_seq (self) } forward_to_deserialize_any ! { bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option seq bytes byte_buf map unit_struct newtype_struct struct identifier tuple enum ignored_any tuple_struct } }
};
}
