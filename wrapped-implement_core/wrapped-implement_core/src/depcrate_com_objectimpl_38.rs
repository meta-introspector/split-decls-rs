// Generated macro for impl_38 (impl)
macro_rules! Depcrate_com_objectimpl_38 {
() => {
// Module: crate::com_object
// Provides: {"impl_38"}
// Dependencies: {}
impl Tombstone { fn is_dead (& self) -> bool { self . cell . load (SeqCst) } fn mark_dead (& self) { self . cell . store (true , SeqCst) ; } }
};
}
