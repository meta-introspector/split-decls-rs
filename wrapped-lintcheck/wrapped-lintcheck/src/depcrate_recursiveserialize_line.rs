// Generated macro for serialize_line (function)
macro_rules! Depcrate_recursiveserialize_line {
() => {
// Module: crate::recursive
// Provides: {"serialize_line"}
// Dependencies: {}
pub (crate) fn serialize_line < T , W > (value : & T , writer : & mut W) where T : Serialize , W : Write , { let mut buf = serde_json :: to_vec (& value) . expect ("failed to serialize") ; buf . push (b'\n') ; writer . write_all (& buf) . expect ("write_all failed") ; }
};
}
