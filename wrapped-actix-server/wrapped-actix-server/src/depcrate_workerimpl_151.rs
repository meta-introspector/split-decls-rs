// Generated macro for impl_151 (impl)
macro_rules! Depcrate_workerimpl_151 {
() => {
// Module: crate::worker
// Provides: {"impl_151"}
// Dependencies: {}
impl WorkerHandleAccept { # [inline (always)] pub (crate) fn idx (& self) -> usize { self . idx } # [inline (always)] pub (crate) fn send (& self , conn : Conn) -> Result < () , Conn > { self . conn_tx . send (conn) . map_err (| msg | msg . 0) } # [inline (always)] pub (crate) fn inc_counter (& self) -> bool { self . counter . inc () } }
};
}
