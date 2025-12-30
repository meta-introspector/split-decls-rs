// Generated macro for impl_124 (impl)
macro_rules! Depcrate_sliceimpl_124 {
() => {
// Module: crate::slice
// Provides: {"impl_124"}
// Dependencies: {}
impl < I : Idx , T : fmt :: Debug > fmt :: Debug for IndexSlice < I , T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . raw , fmt) } }
};
}
