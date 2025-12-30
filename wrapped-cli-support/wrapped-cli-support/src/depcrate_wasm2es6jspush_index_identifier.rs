// Generated macro for push_index_identifier (function)
macro_rules! Depcrate_wasm2es6jspush_index_identifier {
() => {
// Module: crate::wasm2es6js
// Provides: {"push_index_identifier"}
// Dependencies: {}
fn push_index_identifier (i : usize , s : & mut String) { let letter = b'a' + ((i % 26) as u8) ; s . push (letter as char) ; if i >= 26 { write ! (s , "{}" , i / 26) . unwrap () ; } }
};
}
