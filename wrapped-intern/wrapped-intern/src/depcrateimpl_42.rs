// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl < T : Internable + ? Sized > Drop for Interned < T > { # [inline] fn drop (& mut self) { if Arc :: count (& self . arc) == 2 { self . drop_slow () ; } } }
};
}
