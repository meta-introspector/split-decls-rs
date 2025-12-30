// Generated macro for impl_191 (impl)
macro_rules! Depcrate_inputimpl_191 {
() => {
// Module: crate::input
// Provides: {"impl_191"}
// Dependencies: {}
impl < 'a , T : Input > Input for & 'a T { fn at (& self , i : usize) -> InputAt { (* * self) . at (i) } fn next_char (& self , at : InputAt) -> Char { (* * self) . next_char (at) } fn previous_char (& self , at : InputAt) -> Char { (* * self) . previous_char (at) } fn is_empty_match (& self , at : InputAt , empty : & InstEmptyLook) -> bool { (* * self) . is_empty_match (at , empty) } fn prefix_at (& self , prefixes : & LiteralSearcher , at : InputAt ,) -> Option < InputAt > { (* * self) . prefix_at (prefixes , at) } fn len (& self) -> usize { (* * self) . len () } fn as_bytes (& self) -> & [u8] { (* * self) . as_bytes () } }
};
}
