// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < A : Automaton > fmt :: Write for Matcher < A > { fn write_str (& mut self , s : & str) -> fmt :: Result { for & byte in s . as_bytes () { self . advance (byte) ; if self . automaton . is_dead_state (self . state) { break ; } } Ok (()) } }
};
}
