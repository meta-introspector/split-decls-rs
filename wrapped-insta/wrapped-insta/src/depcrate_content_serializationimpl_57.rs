// Generated macro for impl_57 (impl)
macro_rules! Depcrate_content_serializationimpl_57 {
() => {
// Module: crate::content::serialization
// Provides: {"impl_57"}
// Dependencies: {}
impl Content { pub (crate) fn as_key (& self) -> Key < '_ > { match * self . resolve_inner () { Content :: Bool (val) => Key :: Bool (val) , Content :: Char (val) => Key :: U64 (val as u64) , Content :: U16 (val) => Key :: U64 (val . into ()) , Content :: U32 (val) => Key :: U64 (val . into ()) , Content :: U64 (val) => Key :: U64 (val) , Content :: U128 (val) => Key :: U128 (val) , Content :: I16 (val) => Key :: I64 (val . into ()) , Content :: I32 (val) => Key :: I64 (val . into ()) , Content :: I64 (val) => Key :: I64 (val) , Content :: I128 (val) => Key :: I128 (val) , Content :: F32 (val) => Key :: F64 (val . into ()) , Content :: F64 (val) => Key :: F64 (val) , Content :: String (ref val) => Key :: Str (val . as_str ()) , Content :: Bytes (ref val) => Key :: Bytes (& val [..]) , _ => Key :: Other , } } pub (crate) fn sort_maps (& mut self) { self . walk (& mut | content | { if let Content :: Map (ref mut items) = content { items . sort_by (| a , b | match (a . 0 . as_key () , b . 0 . as_key ()) { (Key :: Other , _) | (_ , Key :: Other) => { a . 0 . partial_cmp (& b . 0) . unwrap_or (Ordering :: Equal) } (ref a , ref b) => a . cmp (b) , }) } true }) } }
};
}
