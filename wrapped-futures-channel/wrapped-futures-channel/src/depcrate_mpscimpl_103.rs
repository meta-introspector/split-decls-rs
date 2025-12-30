// Generated macro for impl_103 (impl)
macro_rules! Depcrate_mpscimpl_103 {
() => {
// Module: crate::mpsc
// Provides: {"impl_103"}
// Dependencies: {}
impl < T > fmt :: Debug for Sender < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Sender") . field ("closed" , & self . is_closed ()) . finish () } }
};
}
