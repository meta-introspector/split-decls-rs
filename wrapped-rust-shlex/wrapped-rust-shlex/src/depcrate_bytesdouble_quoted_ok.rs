// Generated macro for double_quoted_ok (function)
macro_rules! Depcrate_bytesdouble_quoted_ok {
() => {
// Module: crate::bytes
// Provides: {"double_quoted_ok"}
// Dependencies: {}
# [doc = " Is this ASCII byte okay to emit in double quotes?"] fn double_quoted_ok (c : u8) -> bool { match c { b'`' | b'$' => false , b'!' | b'^' => false , _ => true } }
};
}
