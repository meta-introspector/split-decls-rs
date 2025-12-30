// Generated macro for impl_193 (impl)
macro_rules! Depcrate_numimpl_193 {
() => {
// Module: crate::num
// Provides: {"impl_193"}
// Dependencies: {}
impl NonZeroChar { pub fn new (ch : char) -> Option < Self > { if ch == '\0' { None } else { Some (NonZeroChar (ch)) } } pub fn get (self) -> char { self . 0 } }
};
}
