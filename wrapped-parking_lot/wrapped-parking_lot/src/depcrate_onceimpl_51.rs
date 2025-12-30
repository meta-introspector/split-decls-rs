// Generated macro for impl_51 (impl)
macro_rules! Depcrate_onceimpl_51 {
() => {
// Module: crate::once
// Provides: {"impl_51"}
// Dependencies: {}
impl fmt :: Debug for Once { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Once") . field ("state" , & self . state ()) . finish () } }
};
}
