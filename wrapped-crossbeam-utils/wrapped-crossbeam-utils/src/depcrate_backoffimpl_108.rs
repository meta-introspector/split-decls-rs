// Generated macro for impl_108 (impl)
macro_rules! Depcrate_backoffimpl_108 {
() => {
// Module: crate::backoff
// Provides: {"impl_108"}
// Dependencies: {}
impl fmt :: Debug for Backoff { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Backoff") . field ("step" , & self . step) . field ("is_completed" , & self . is_completed ()) . finish () } }
};
}
