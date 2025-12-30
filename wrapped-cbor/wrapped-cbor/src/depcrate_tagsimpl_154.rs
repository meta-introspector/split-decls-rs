// Generated macro for impl_154 (impl)
macro_rules! Depcrate_tagsimpl_154 {
() => {
// Module: crate::tags
// Provides: {"impl_154"}
// Dependencies: {}
impl < 'de , 'a , E > Deserializer < 'de > for BytesDeserializer < 'a , E > where E : serde :: de :: Error , { type Error = E ; fn deserialize_any < V : Visitor < 'de > > (self , visitor : V) -> Result < V :: Value , Self :: Error > { visitor . visit_bytes (self . 0) } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any } }
};
}
