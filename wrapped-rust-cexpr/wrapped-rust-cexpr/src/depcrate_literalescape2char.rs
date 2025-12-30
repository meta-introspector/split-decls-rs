// Generated macro for escape2char (function)
macro_rules! Depcrate_literalescape2char {
() => {
// Module: crate::literal
// Provides: {"escape2char"}
// Dependencies: {}
fn escape2char (c : char) -> CChar { CChar :: Char (match c { 'a' => '\x07' , 'b' => '\x08' , 'f' => '\x0c' , 'n' => '\n' , 'r' => '\r' , 't' => '\t' , 'v' => '\x0b' , _ => unreachable ! ("invalid escape {}" , c) , }) }
};
}
