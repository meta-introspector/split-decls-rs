// Generated macro for impl_109 (impl)
macro_rules! Depcrate_ext_deimpl_109 {
() => {
// Module: crate::ext::de
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'de , I , U > Deserializer < 'de > for SeqDeserializer < I > where I : ExactSizeIterator < Item = U > , U : Deserializer < 'de , Error = Error > { type Error = Error ; # [inline] fn deserialize_any < V > (mut self , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > { let len = self . iter . len () ; if len == 0 { visitor . visit_unit () } else { let ret = visitor . visit_seq (& mut self) ? ; let rem = self . iter . len () ; if rem == 0 { Ok (ret) } else { Err (de :: Error :: invalid_length (len , & "fewer elements in array")) } } } forward_to_deserialize_any ! { bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option seq bytes byte_buf map unit_struct newtype_struct tuple_struct struct identifier tuple enum ignored_any } }
};
}
