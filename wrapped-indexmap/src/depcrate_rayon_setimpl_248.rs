// Generated macro for impl_248 (impl)
macro_rules! Depcrate_rayon_setimpl_248 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_248"}
// Dependencies: {}
impl < T , S1 , S2 > fmt :: Debug for ParIntersection < '_ , T , S1 , S2 > where T : fmt :: Debug + Eq + Hash , S1 : BuildHasher , S2 : BuildHasher , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . set1 . intersection (self . set2)) . finish () } }
};
}
