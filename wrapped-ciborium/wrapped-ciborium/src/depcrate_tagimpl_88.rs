// Generated macro for impl_88 (impl)
macro_rules! Depcrate_tagimpl_88 {
() => {
// Module: crate::tag
// Provides: {"impl_88"}
// Dependencies: {}
impl < 'de , D : de :: Deserializer < 'de > > de :: Deserializer < 'de > for & mut TagAccess < D > { type Error = D :: Error ; # [inline] fn deserialize_any < V : de :: Visitor < 'de > > (self , visitor : V) -> Result < V :: Value , Self :: Error > { self . state += 1 ; match self . state { 1 => visitor . visit_str (match self . tag { Some (..) => "@@TAGGED@@" , None => "@@UNTAGGED@@" , }) , _ => visitor . visit_u64 (self . tag . unwrap ()) , } } forward_to_deserialize_any ! { i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 bool f32 f64 char str string bytes byte_buf seq map struct tuple tuple_struct identifier ignored_any option unit unit_struct newtype_struct enum } }
};
}
