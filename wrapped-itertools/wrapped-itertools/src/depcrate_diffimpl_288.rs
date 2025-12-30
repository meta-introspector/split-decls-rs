// Generated macro for impl_288 (impl)
macro_rules! Depcrate_diffimpl_288 {
() => {
// Module: crate::diff
// Provides: {"impl_288"}
// Dependencies: {}
impl < I , J > fmt :: Debug for Diff < I , J > where I : Iterator , J : Iterator , PutBack < I > : fmt :: Debug , PutBack < J > : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: FirstMismatch (idx , i , j) => f . debug_tuple ("FirstMismatch") . field (idx) . field (i) . field (j) . finish () , Self :: Shorter (idx , i) => f . debug_tuple ("Shorter") . field (idx) . field (i) . finish () , Self :: Longer (idx , j) => f . debug_tuple ("Longer") . field (idx) . field (j) . finish () , } } }
};
}
