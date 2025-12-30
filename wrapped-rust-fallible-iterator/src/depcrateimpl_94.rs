// Generated macro for impl_94 (impl)
macro_rules! Depcrateimpl_94 {
() => {
// Module: crate
// Provides: {"impl_94"}
// Dependencies: {}
impl < I > Clone for Flatten < I > where I : FallibleIterator + Clone , I :: Item : IntoFallibleIterator , < I :: Item as IntoFallibleIterator > :: IntoFallibleIter : Clone , { # [inline] fn clone (& self) -> Flatten < I > { Flatten { it : self . it . clone () , cur : self . cur . clone () , } } }
};
}
