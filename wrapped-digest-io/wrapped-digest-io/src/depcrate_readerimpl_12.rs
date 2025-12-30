// Generated macro for impl_12 (impl)
macro_rules! Depcrate_readerimpl_12 {
() => {
// Module: crate::reader
// Provides: {"impl_12"}
// Dependencies: {}
impl < D : Digest + Clone , R : io :: Read + Clone > Clone for HashReader < D , R > { fn clone (& self) -> HashReader < D , R > { HashReader { reader : self . reader . clone () , hasher : self . hasher . clone () , } } }
};
}
