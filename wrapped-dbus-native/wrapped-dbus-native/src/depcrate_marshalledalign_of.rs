// Generated macro for align_of (function)
macro_rules! Depcrate_marshalledalign_of {
() => {
// Module: crate::marshalled
// Provides: {"align_of"}
// Dependencies: {}
pub fn align_of (c : u8) -> usize { match c { b'y' | b'g' | b'v' => 1 , b'n' | b'q' => 2 , b'i' | b'u' | b'b' | b's' | b'o' | b'a' | b'h' => 4 , b'x' | b't' | b'd' | b'(' | b'{' => 8 , _ => panic ! ("Unexpected byte in type signature: {}" , c) } }
};
}
