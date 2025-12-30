// Generated macro for impl_20 (impl)
macro_rules! Depcrate_filterimpl_20 {
() => {
// Module: crate::filter
// Provides: {"impl_20"}
// Dependencies: {}
impl fmt :: Debug for Builder { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . built { f . debug_struct ("Filter") . field ("built" , & true) . finish () } else { f . debug_struct ("Filter") . field ("filter" , & self . filter) . field ("directives" , & self . directives) . finish () } } }
};
}
