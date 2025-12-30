// Generated macro for impl_144 (impl)
macro_rules! Depcrate_vecimpl_144 {
() => {
// Module: crate::vec
// Provides: {"impl_144"}
// Dependencies: {}
impl < I : Idx , T : fmt :: Debug > fmt :: Debug for IndexVec < I , T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . raw , fmt) } }
};
}
