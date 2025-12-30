// Generated macro for impl_44 (impl)
macro_rules! Depcrate_systemimpl_44 {
() => {
// Module: crate::system
// Provides: {"impl_44"}
// Dependencies: {}
impl SystemController { pub (crate) fn new (cmd_rx : mpsc :: UnboundedReceiver < SystemCommand > , stop_tx : oneshot :: Sender < i32 > ,) -> Self { SystemController { cmd_rx , stop_tx : Some (stop_tx) , arbiters : HashMap :: with_capacity (4) , } } }
};
}
