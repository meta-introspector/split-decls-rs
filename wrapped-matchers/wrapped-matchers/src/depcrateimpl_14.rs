// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl < A : Automaton > io :: Write for Matcher < A > { fn write (& mut self , bytes : & [u8]) -> Result < usize , io :: Error > { let mut i = 0 ; for & byte in bytes { self . advance (byte) ; i += 1 ; if self . automaton . is_dead_state (self . state) { break ; } } Ok (i) } fn flush (& mut self) -> Result < () , io :: Error > { Ok (()) } }
};
}
