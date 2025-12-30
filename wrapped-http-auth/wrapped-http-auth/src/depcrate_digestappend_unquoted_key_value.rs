// Generated macro for append_unquoted_key_value (function)
macro_rules! Depcrate_digestappend_unquoted_key_value {
() => {
// Module: crate::digest
// Provides: {"append_unquoted_key_value"}
// Dependencies: {}
# [inline (never)] fn append_unquoted_key_value (out : & mut String , key : & str , value : & str) { out . push_str (key) ; out . push ('=') ; out . push_str (value) ; out . push_str (", ") ; }
};
}
