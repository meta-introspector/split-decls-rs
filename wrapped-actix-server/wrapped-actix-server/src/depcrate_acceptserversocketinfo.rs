// Generated macro for ServerSocketInfo (struct)
macro_rules! Depcrate_acceptServerSocketInfo {
() => {
// Module: crate::accept
// Provides: {"ServerSocketInfo"}
// Dependencies: {}
struct ServerSocketInfo { token : usize , lst : MioListener , # [doc = " Timeout is used to mark the deadline when this socket's listener should be registered again"] # [doc = " after an error."] timeout : Option < actix_rt :: time :: Instant > , }
};
}
