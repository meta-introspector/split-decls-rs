// Generated macro for impl_191 (impl)
macro_rules! Depcrate_safeimpl_191 {
() => {
// Module: crate::safe
// Provides: {"impl_191"}
// Dependencies: {}
impl < 'lib , T > Clone for Symbol < 'lib , T > { fn clone (& self) -> Symbol < 'lib , T > { Symbol { inner : self . inner . clone () , pd : marker :: PhantomData , } } }
};
}
