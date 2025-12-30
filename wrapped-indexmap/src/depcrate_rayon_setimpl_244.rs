// Generated macro for impl_244 (impl)
macro_rules! Depcrate_rayon_setimpl_244 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_244"}
// Dependencies: {}
impl < T , S1 , S2 > fmt :: Debug for ParDifference < '_ , T , S1 , S2 > where T : fmt :: Debug + Eq + Hash , S1 : BuildHasher , S2 : BuildHasher , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . set1 . difference (self . set2)) . finish () } }
};
}
