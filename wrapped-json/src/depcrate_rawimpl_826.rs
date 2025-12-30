// Generated macro for impl_826 (impl)
macro_rules! Depcrate_rawimpl_826 {
() => {
// Module: crate::raw
// Provides: {"impl_826"}
// Dependencies: {}
impl < 'de > Deserializer < 'de > for RawKeyDeserializer { type Error = Error ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Error > where V : de :: Visitor < 'de > , { visitor . visit_borrowed_str (TOKEN) } forward_to_deserialize_any ! { bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 char str string seq bytes byte_buf map struct option unit newtype_struct ignored_any unit_struct tuple_struct tuple enum identifier } }
};
}
