// Generated macro for impl_104 (impl)
macro_rules! Depcrate_mpscimpl_104 {
() => {
// Module: crate::mpsc
// Provides: {"impl_104"}
// Dependencies: {}
impl < T > fmt :: Debug for UnboundedSender < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("UnboundedSender") . field ("closed" , & self . is_closed ()) . finish () } }
};
}
