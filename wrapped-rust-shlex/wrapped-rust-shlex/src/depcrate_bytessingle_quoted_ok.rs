// Generated macro for single_quoted_ok (function)
macro_rules! Depcrate_bytessingle_quoted_ok {
() => {
// Module: crate::bytes
// Provides: {"single_quoted_ok"}
// Dependencies: {}
# [doc = " Is this ASCII byte okay to emit in single quotes?"] fn single_quoted_ok (c : u8) -> bool { match c { b'\'' => false , b'^' => false , b'\\' => false , _ => true } }
};
}
