// Generated macro for impl_348 (impl)
macro_rules! Depcrate_value_deimpl_348 {
() => {
// Module: crate::value::de
// Provides: {"impl_348"}
// Dependencies: {}
impl < 'de > serde :: Deserializer < 'de > for Map < String , Value > { type Error = Error ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { let len = self . len () ; let mut deserializer = MapDeserializer :: new (self) ; let map = tri ! (visitor . visit_map (& mut deserializer)) ; let remaining = deserializer . iter . len () ; if remaining == 0 { Ok (map) } else { Err (serde :: de :: Error :: invalid_length (len , & "fewer elements in map" ,)) } } fn deserialize_enum < V > (self , _name : & 'static str , _variants : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { let mut iter = self . into_iter () ; let (variant , value) = match iter . next () { Some (v) => v , None => { return Err (serde :: de :: Error :: invalid_value (Unexpected :: Map , & "map with a single key" ,)) ; } } ; if iter . next () . is_some () { return Err (serde :: de :: Error :: invalid_value (Unexpected :: Map , & "map with a single key" ,)) ; } visitor . visit_enum (EnumDeserializer { variant , value : Some (value) , }) } fn deserialize_ignored_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { drop (self) ; visitor . visit_unit () } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct identifier } }
};
}
