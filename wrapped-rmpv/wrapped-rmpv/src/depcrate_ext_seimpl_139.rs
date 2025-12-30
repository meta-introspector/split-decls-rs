// Generated macro for impl_139 (impl)
macro_rules! Depcrate_ext_seimpl_139 {
() => {
// Module: crate::ext::se
// Provides: {"impl_139"}
// Dependencies: {}
impl Serialize for Value { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer { match * self { Self :: Nil => s . serialize_unit () , Self :: Boolean (v) => s . serialize_bool (v) , Self :: Integer (Integer { n }) => match n { IntPriv :: PosInt (n) => s . serialize_u64 (n) , IntPriv :: NegInt (n) => s . serialize_i64 (n) , } , Self :: F32 (v) => s . serialize_f32 (v) , Self :: F64 (v) => s . serialize_f64 (v) , Self :: String (ref v) => match v . s { Ok (ref v) => s . serialize_str (v) , Err (ref v) => Bytes :: new (& v . 0 [..]) . serialize (s) , } , Self :: Binary (ref v) => Bytes :: new (& v [..]) . serialize (s) , Self :: Array (ref array) => { let mut state = s . serialize_seq (Some (array . len ())) ? ; for item in array { state . serialize_element (item) ? ; } state . end () } Self :: Map (ref map) => { let mut state = s . serialize_map (Some (map . len ())) ? ; for (key , val) in map { state . serialize_entry (key , val) ? ; } state . end () } Self :: Ext (ty , ref buf) => { let value = (ty , Bytes :: new (& buf [..])) ; s . serialize_newtype_struct (MSGPACK_EXT_STRUCT_NAME , & value) } } } }
};
}
