// Generated macro for byte (function)
macro_rules! Depcrate_write_stringbyte {
() => {
// Module: crate::write::string
// Provides: {"byte"}
// Dependencies: {}
fn byte (id : usize , pos : usize , strings : & IndexSet < & [u8] >) -> u8 { let string = strings . get_index (id) . unwrap () ; let len = string . len () ; if len >= pos { string [len - pos] } else { 0 } }
};
}
