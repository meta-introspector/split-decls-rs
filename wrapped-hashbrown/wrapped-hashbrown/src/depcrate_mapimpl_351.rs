// Generated macro for impl_351 (impl)
macro_rules! Depcrate_mapimpl_351 {
() => {
// Module: crate::map
// Provides: {"impl_351"}
// Dependencies: {}
impl < K : Debug , V : Debug , S , A : Allocator > Debug for OccupiedEntry < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("key" , self . key ()) . field ("value" , self . get ()) . finish () } }
};
}
