// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
impl < T : Internable + ? Sized > Deref for Interned < T > { type Target = T ; # [inline] fn deref (& self) -> & Self :: Target { & self . arc } }
};
}
