// Generated macro for impl_615 (impl)
macro_rules! Depcrate_nfa_thompson_nfaimpl_615 {
() => {
// Module: crate::nfa::thompson::nfa
// Provides: {"impl_615"}
// Dependencies: {}
impl Transition { # [doc = " Returns true if the position `at` in `haystack` falls in this"] # [doc = " transition's range of bytes."] # [doc = ""] # [doc = " If `at >= haystack.len()`, then this returns `false`."] pub fn matches (& self , haystack : & [u8] , at : usize) -> bool { haystack . get (at) . map_or (false , | & b | self . matches_byte (b)) } # [doc = " Returns true if the given alphabet unit falls in this transition's"] # [doc = " range of bytes. If the given unit is [`EOI`](alphabet::Unit::eoi), then"] # [doc = " this returns `false`."] pub fn matches_unit (& self , unit : alphabet :: Unit) -> bool { unit . as_u8 () . map_or (false , | byte | self . matches_byte (byte)) } # [doc = " Returns true if the given byte falls in this transition's range of"] # [doc = " bytes."] pub fn matches_byte (& self , byte : u8) -> bool { self . start <= byte && byte <= self . end } }
};
}
