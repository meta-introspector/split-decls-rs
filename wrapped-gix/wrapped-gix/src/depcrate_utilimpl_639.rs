// Generated macro for impl_639 (impl)
macro_rules! Depcrate_utilimpl_639 {
() => {
// Module: crate::util
// Provides: {"impl_639"}
// Dependencies: {}
impl Deref for OwnedOrStaticAtomicBool { type Target = std :: sync :: atomic :: AtomicBool ; fn deref (& self) -> & Self :: Target { match self { OwnedOrStaticAtomicBool :: Owned { flag , .. } => flag , OwnedOrStaticAtomicBool :: Shared (flag) => flag , } } }
};
}
