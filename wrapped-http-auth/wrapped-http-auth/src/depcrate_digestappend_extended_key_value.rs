// Generated macro for append_extended_key_value (function)
macro_rules! Depcrate_digestappend_extended_key_value {
() => {
// Module: crate::digest
// Provides: {"append_extended_key_value"}
// Dependencies: {}
fn append_extended_key_value (out : & mut String , key : & str , value : & str) { out . push_str (key) ; out . push_str ("*=UTF-8''") ; for & b in value . as_bytes () { if (char_classes (b) & C_ATTR) != 0 { out . push (char :: from (b)) ; } else { let _ = write ! (out , "%{:02X}" , b) ; } } out . push_str (", ") ; }
};
}
