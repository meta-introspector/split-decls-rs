// Generated macro for impl_1223 (impl)
macro_rules! Depcrate_graphmapimpl_1223 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1223"}
// Dependencies: {}
impl < T > Hash for Ptr < '_ , T > { fn hash < H : hash :: Hasher > (& self , st : & mut H) { let ptr = (self . 0) as * const T ; ptr . hash (st) } }
};
}
