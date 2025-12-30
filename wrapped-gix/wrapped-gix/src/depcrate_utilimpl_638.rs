// Generated macro for impl_638 (impl)
macro_rules! Depcrate_utilimpl_638 {
() => {
// Module: crate::util
// Provides: {"impl_638"}
// Dependencies: {}
impl Default for OwnedOrStaticAtomicBool { fn default () -> Self { OwnedOrStaticAtomicBool :: Owned { flag : Arc :: new (AtomicBool :: default ()) , private : true , } } }
};
}
