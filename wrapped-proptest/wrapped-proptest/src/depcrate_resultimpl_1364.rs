// Generated macro for impl_1364 (impl)
macro_rules! Depcrate_resultimpl_1364 {
() => {
// Module: crate::result
// Provides: {"impl_1364"}
// Dependencies: {}
impl < T : Strategy , E : Strategy > Clone for MaybeErrValueTree < T , E > where T :: Tree : Clone , E :: Tree : Clone , { fn clone (& self) -> Self { MaybeErrValueTree (self . 0 . clone ()) } }
};
}
