// Generated macro for basic_type (function)
macro_rules! Depcrate_v0basic_type {
() => {
// Module: crate::v0
// Provides: {"basic_type"}
// Dependencies: {}
fn basic_type (tag : u8) -> Option < & 'static str > { Some (match tag { b'b' => "bool" , b'c' => "char" , b'e' => "str" , b'u' => "()" , b'a' => "i8" , b's' => "i16" , b'l' => "i32" , b'x' => "i64" , b'n' => "i128" , b'i' => "isize" , b'h' => "u8" , b't' => "u16" , b'm' => "u32" , b'y' => "u64" , b'o' => "u128" , b'j' => "usize" , b'f' => "f32" , b'd' => "f64" , b'z' => "!" , b'p' => "_" , b'v' => "..." , _ => return None , }) }
};
}
