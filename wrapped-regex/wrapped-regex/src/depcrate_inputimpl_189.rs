// Generated macro for impl_189 (impl)
macro_rules! Depcrate_inputimpl_189 {
() => {
// Module: crate::input
// Provides: {"impl_189"}
// Dependencies: {}
impl InputAt { # [doc = " Returns true iff this position is at the beginning of the input."] pub fn is_start (& self) -> bool { self . pos == 0 } # [doc = " Returns true iff this position is past the end of the input."] pub fn is_end (& self) -> bool { self . c . is_none () && self . byte . is_none () } # [doc = " Returns the character at this position."] # [doc = ""] # [doc = " If this position is just before or after the input, then an absent"] # [doc = " character is returned."] pub fn char (& self) -> Char { self . c } # [doc = " Returns the byte at this position."] pub fn byte (& self) -> Option < u8 > { self . byte } # [doc = " Returns the UTF-8 width of the character at this position."] pub fn len (& self) -> usize { self . len } # [doc = " Returns the byte offset of this position."] pub fn pos (& self) -> usize { self . pos } # [doc = " Returns the byte offset of the next position in the input."] pub fn next_pos (& self) -> usize { self . pos + self . len } }
};
}
