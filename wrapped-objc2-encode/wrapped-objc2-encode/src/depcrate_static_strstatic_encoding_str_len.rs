// Generated macro for static_encoding_str_len (function)
macro_rules! Depcrate_static_strstatic_encoding_str_len {
() => {
// Module: crate::static_str
// Provides: {"static_encoding_str_len"}
// Dependencies: {}
pub (crate) const fn static_encoding_str_len (encoding : & Encoding , level : NestingLevel) -> usize { use Helper :: * ; match Helper :: new (encoding) { Primitive (primitive) => primitive . to_str () . len () , BitField (size , None) => 1 + static_int_str_len (size as u64) , BitField (size , Some ((offset , t))) => { 1 + static_int_str_len (* offset) + static_encoding_str_len (t , level . bitfield ()) + static_int_str_len (size as u64) } Indirection (kind , t) => 1 + static_encoding_str_len (t , level . indirection (kind)) , Array (len , item) => { 1 + static_int_str_len (len) + static_encoding_str_len (item , level . array ()) + 1 } Container (_ , name , items) => { let mut res = 1 + name . len () ; if let Some (level) = level . container_include_fields () { res += 1 ; let mut i = 0 ; while i < items . len () { res += static_encoding_str_len (& items [i] , level) ; i += 1 ; } } res + 1 } NoneInvalid => 0 , } }
};
}
