// Generated macro for impl_11 (impl)
macro_rules! Depcrate_loggerimpl_11 {
() => {
// Module: crate::logger
// Provides: {"impl_11"}
// Dependencies: {}
impl std :: fmt :: Debug for Builder { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if self . built { f . debug_struct ("Logger") . field ("built" , & true) . finish () } else { f . debug_struct ("Logger") . field ("filter" , & self . filter) . field ("writer" , & self . writer) . finish () } } }
};
}
