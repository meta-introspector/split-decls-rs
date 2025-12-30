// Generated macro for impl_641 (impl)
macro_rules! Depcrate_utilimpl_641 {
() => {
// Module: crate::util
// Provides: {"impl_641"}
// Dependencies: {}
impl < 'a > From < & 'a Arc < AtomicBool > > for OwnedOrStaticAtomicBool { fn from (value : & 'a Arc < AtomicBool >) -> Self { OwnedOrStaticAtomicBool :: Owned { flag : value . clone () , private : false , } } }
};
}
