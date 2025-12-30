// Generated macro for impl_104 (impl)
macro_rules! Depcrate_ext_deimpl_104 {
() => {
// Module: crate::ext::de
// Provides: {"impl_104"}
// Dependencies: {}
# [doc = " Deserializer for Ext (expecting sequence)"] impl < 'a , 'de : 'a > Deserializer < 'de > for ExtDeserializer < 'de > { type Error = Error ; # [inline] fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > { visitor . visit_seq (self) } forward_to_deserialize_any ! { bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option seq bytes byte_buf map unit_struct newtype_struct struct identifier tuple enum ignored_any tuple_struct } }
};
}
