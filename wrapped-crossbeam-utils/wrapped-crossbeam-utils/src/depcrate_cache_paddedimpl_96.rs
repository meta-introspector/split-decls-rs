// Generated macro for impl_96 (impl)
macro_rules! Depcrate_cache_paddedimpl_96 {
() => {
// Module: crate::cache_padded
// Provides: {"impl_96"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for CachePadded < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("CachePadded") . field ("value" , & self . value) . finish () } }
};
}
