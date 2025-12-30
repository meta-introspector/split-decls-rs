// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl < T : Collect > ErasedNode for T { unsafe fn submit (& self , node : & 'static Node) { unsafe { T :: registry () . submit (node) ; } } }
};
}
