// Generated macro for impl_17 (impl)
macro_rules! Depcrate_bytesimpl_17 {
() => {
// Module: crate::bytes
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'a > Iterator for Shlex < 'a > { type Item = Vec < u8 > ; fn next (& mut self) -> Option < Self :: Item > { if let Some (mut ch) = self . next_char () { loop { match ch as char { ' ' | '\t' | '\n' => { } , '#' => { while let Some (ch2) = self . next_char () { if ch2 as char == '\n' { break ; } } } , _ => { break ; } } if let Some (ch2) = self . next_char () { ch = ch2 ; } else { return None ; } } self . parse_word (ch) } else { None } } }
};
}
