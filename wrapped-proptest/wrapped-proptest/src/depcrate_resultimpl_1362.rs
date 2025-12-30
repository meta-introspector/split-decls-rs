// Generated macro for impl_1362 (impl)
macro_rules! Depcrate_resultimpl_1362 {
() => {
// Module: crate::result
// Provides: {"impl_1362"}
// Dependencies: {}
impl < T : Strategy , E : Strategy > Clone for MaybeOkValueTree < T , E > where T :: Tree : Clone , E :: Tree : Clone , { fn clone (& self) -> Self { MaybeOkValueTree (self . 0 . clone ()) } }
};
}
