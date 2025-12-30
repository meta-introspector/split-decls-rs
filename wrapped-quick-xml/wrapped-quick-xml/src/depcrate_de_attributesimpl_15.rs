// Generated macro for impl_15 (impl)
macro_rules! Depcrate_de_attributesimpl_15 {
() => {
// Module: crate::de::attributes
// Provides: {"impl_15"}
// Dependencies: {}
impl < 'de > Deserializer < 'de > for AttributesDeserializer < 'de > { type Error = DeError ; # [inline] fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { visitor . visit_map (self) } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any } }
};
}
