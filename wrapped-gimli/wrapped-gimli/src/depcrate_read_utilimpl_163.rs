// Generated macro for impl_163 (impl)
macro_rules! Depcrate_read_utilimpl_163 {
() => {
// Module: crate::read::util
// Provides: {"impl_163"}
// Dependencies: {}
impl < A : ArrayLike > Clone for ArrayVec < A > where A :: Item : Clone , { fn clone (& self) -> Self { let mut new = Self :: default () ; for value in & * * self { new . try_push (value . clone ()) . unwrap () ; } new } }
};
}
