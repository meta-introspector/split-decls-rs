// Generated macro for impl_394 (impl)
macro_rules! Depcrate_mapimpl_394 {
() => {
// Module: crate::map
// Provides: {"impl_394"}
// Dependencies: {}
impl < K , V , A > fmt :: Debug for Drain < '_ , K , V , A > where K : fmt :: Debug , V : fmt :: Debug , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
