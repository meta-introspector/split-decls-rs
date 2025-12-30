// Generated macro for impl_119 (impl)
macro_rules! Depcrate_mpscimpl_119 {
() => {
// Module: crate::mpsc
// Provides: {"impl_119"}
// Dependencies: {}
impl < T > fmt :: Debug for UnboundedReceiver < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let closed = if let Some (ref inner) = self . inner { decode_state (inner . state . load (SeqCst)) . is_closed () } else { false } ; f . debug_struct ("Receiver") . field ("closed" , & closed) . finish () } }
};
}
