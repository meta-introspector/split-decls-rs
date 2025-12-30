// Generated macro for impl_256 (impl)
macro_rules! Depcrate_rayon_setimpl_256 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_256"}
// Dependencies: {}
impl < T , S1 , S2 > fmt :: Debug for ParUnion < '_ , T , S1 , S2 > where T : fmt :: Debug + Eq + Hash , S1 : BuildHasher , S2 : BuildHasher , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . set1 . union (self . set2)) . finish () } }
};
}
