// Generated macro for impl_15 (impl)
macro_rules! Depcrate_loggerimpl_15 {
() => {
// Module: crate::logger
// Provides: {"impl_15"}
// Dependencies: {}
impl std :: fmt :: Debug for Logger { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Logger") . field ("filter" , & self . filter) . finish () } }
};
}
