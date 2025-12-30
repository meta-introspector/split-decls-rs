// Generated macro for impl_9 (impl)
macro_rules! Depcrate_iterimpl_9 {
() => {
// Module: crate::iter
// Provides: {"impl_9"}
// Dependencies: {}
impl < B : Flags > IterNames < B > { pub (crate) fn new (flags : & B) -> Self { IterNames { flags : B :: FLAGS , idx : 0 , remaining : B :: from_bits_retain (flags . bits ()) , source : B :: from_bits_retain (flags . bits ()) , } } }
};
}
