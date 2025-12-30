// Generated macro for impl_1046 (impl)
macro_rules! Depcrate_transliterate_transliterator_replaceableimpl_1046 {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"impl_1046"}
// Dependencies: {}
impl < F > Drop for InsertableToReplaceableAdapter < '_ , '_ , F > where F : FnMut (usize) , { fn drop (& mut self) { (self . on_drop) (self . child . curr) ; } }
};
}
