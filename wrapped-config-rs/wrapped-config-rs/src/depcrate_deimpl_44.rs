// Generated macro for impl_44 (impl)
macro_rules! Depcrate_deimpl_44 {
() => {
// Module: crate::de
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'de > de :: Deserializer < 'de > for StrDeserializer < '_ > { type Error = ConfigError ; # [inline] fn deserialize_any < V : de :: Visitor < 'de > > (self , visitor : V) -> Result < V :: Value > { visitor . visit_str (self . 0) } serde_core :: forward_to_deserialize_any ! { bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq bytes byte_buf map struct unit enum newtype_struct identifier ignored_any unit_struct tuple_struct tuple option } }
};
}
