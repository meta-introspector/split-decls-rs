// Generated macro for impl_203 (impl)
macro_rules! Depcrate_value_serimpl_203 {
() => {
// Module: crate::value::ser
// Provides: {"impl_203"}
// Dependencies: {}
impl serde :: Serialize for Value { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { match * self { Value :: Integer (v) => serializer . serialize_i128 (v) , Value :: Bytes (ref v) => serializer . serialize_bytes (& v) , Value :: Text (ref v) => serializer . serialize_str (& v) , Value :: Array (ref v) => v . serialize (serializer) , Value :: Map (ref v) => v . serialize (serializer) , Value :: Tag (tag , ref v) => Tagged :: new (Some (tag) , v) . serialize (serializer) , Value :: Float (v) => serializer . serialize_f64 (v) , Value :: Bool (v) => serializer . serialize_bool (v) , Value :: Null => serializer . serialize_unit () , Value :: __Hidden => unreachable ! () , } } }
};
}
