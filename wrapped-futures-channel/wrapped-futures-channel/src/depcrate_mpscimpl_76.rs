// Generated macro for impl_76 (impl)
macro_rules! Depcrate_mpscimpl_76 {
() => {
// Module: crate::mpsc
// Provides: {"impl_76"}
// Dependencies: {}
impl < T > fmt :: Debug for TrySendError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TrySendError") . field ("kind" , & self . err . kind) . finish () } }
};
}
