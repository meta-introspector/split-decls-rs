// Generated macro for impl_22 (impl)
macro_rules! Depcrate_writerimpl_22 {
() => {
// Module: crate::writer
// Provides: {"impl_22"}
// Dependencies: {}
impl < D : Digest + Clone , W : io :: Write + Clone > Clone for HashWriter < D , W > { fn clone (& self) -> HashWriter < D , W > { HashWriter { writer : self . writer . clone () , hasher : self . hasher . clone () , } } }
};
}
