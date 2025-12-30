// Generated macro for impl_1050 (impl)
macro_rules! Depcrate_transliterate_transliterator_replaceableimpl_1050 {
() => {
// Module: crate::transliterate::transliterator::replaceable
// Provides: {"impl_1050"}
// Dependencies: {}
impl < 'a , F > InsertableGuard < 'a , F > where F : FnMut (& [u8]) , { fn new (rep : Replaceable < 'a > , on_drop : F) -> Self { Self { rep , on_drop } } pub (crate) fn child (& mut self) -> Replaceable < '_ > { self . rep . child () } }
};
}
