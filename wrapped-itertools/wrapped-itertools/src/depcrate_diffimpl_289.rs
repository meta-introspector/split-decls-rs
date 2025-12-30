// Generated macro for impl_289 (impl)
macro_rules! Depcrate_diffimpl_289 {
() => {
// Module: crate::diff
// Provides: {"impl_289"}
// Dependencies: {}
impl < I , J > Clone for Diff < I , J > where I : Iterator , J : Iterator , PutBack < I > : Clone , PutBack < J > : Clone , { fn clone (& self) -> Self { match self { Self :: FirstMismatch (idx , i , j) => Self :: FirstMismatch (* idx , i . clone () , j . clone ()) , Self :: Shorter (idx , i) => Self :: Shorter (* idx , i . clone ()) , Self :: Longer (idx , j) => Self :: Longer (* idx , j . clone ()) , } } }
};
}
