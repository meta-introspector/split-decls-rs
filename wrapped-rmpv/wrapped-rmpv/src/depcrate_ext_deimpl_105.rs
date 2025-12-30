// Generated macro for impl_105 (impl)
macro_rules! Depcrate_ext_deimpl_105 {
() => {
// Module: crate::ext::de
// Provides: {"impl_105"}
// Dependencies: {}
# [doc = " Deserializer for Ext `SeqAccess` elements"] impl < 'a , 'de : 'a > Deserializer < 'de > for & 'a mut ExtDeserializer < 'de > { type Error = Error ; # [inline] fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > { if self . tag . is_some () { let tag = self . tag . take () . unwrap () ; visitor . visit_i8 (tag) } else if self . data . is_some () { let data = self . data . take () . unwrap () ; match data { Cow :: Owned (data) => visitor . visit_byte_buf (data) , Cow :: Borrowed (data) => visitor . visit_borrowed_bytes (data) , } } else { debug_assert ! (false , "ext seq only has two elements") ; Err (Error :: Syntax (String :: new ())) } } forward_to_deserialize_any ! { bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option seq bytes byte_buf map unit_struct newtype_struct tuple_struct struct identifier tuple enum ignored_any } }
};
}
