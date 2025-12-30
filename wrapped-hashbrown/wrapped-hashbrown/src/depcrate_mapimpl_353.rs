// Generated macro for impl_353 (impl)
macro_rules! Depcrate_mapimpl_353 {
() => {
// Module: crate::map
// Provides: {"impl_353"}
// Dependencies: {}
impl < K : Debug , V , S , A : Allocator > Debug for VacantEntry < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("VacantEntry") . field (self . key ()) . finish () } }
};
}
