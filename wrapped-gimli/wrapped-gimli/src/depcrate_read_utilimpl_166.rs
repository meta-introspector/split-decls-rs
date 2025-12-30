// Generated macro for impl_166 (impl)
macro_rules! Depcrate_read_utilimpl_166 {
() => {
// Module: crate::read::util
// Provides: {"impl_166"}
// Dependencies: {}
impl < A : ArrayLike > fmt :: Debug for ArrayVec < A > where A :: Item : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
