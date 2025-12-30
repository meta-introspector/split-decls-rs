// Generated macro for impl_595 (impl)
macro_rules! Depcrate_connectionimpl_595 {
() => {
// Module: crate::connection
// Provides: {"impl_595"}
// Dependencies: {}
impl ConnectionSide { fn remote_may_migrate (& self) -> bool { match self { Self :: Server { server_config } => server_config . migration , Self :: Client { .. } => false , } } fn is_client (& self) -> bool { self . side () . is_client () } fn is_server (& self) -> bool { self . side () . is_server () } fn side (& self) -> Side { match * self { Self :: Client { .. } => Side :: Client , Self :: Server { .. } => Side :: Server , } } }
};
}
