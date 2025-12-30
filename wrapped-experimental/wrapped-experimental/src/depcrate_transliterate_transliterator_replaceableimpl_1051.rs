// Generated macro for impl_1051 (impl)
macro_rules! Depcrate_transliterate_transliterator_replaceableimpl_1051 {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"impl_1051"}
// Dependencies: {}
impl < F > Drop for InsertableGuard < '_ , F > where F : FnMut (& [u8]) , { fn drop (& mut self) { (self . on_drop) (& self . rep . content) ; } }
};
}
