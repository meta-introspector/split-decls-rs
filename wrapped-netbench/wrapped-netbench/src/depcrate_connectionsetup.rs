// Generated macro for setup (function)
macro_rules! Depcrate_connectionsetup {
() => {
// Module: crate::connection
// Provides: {"setup"}
// Dependencies: {}
# [doc = " Setup the streams and eventually pins the thread according to the configuration."] pub fn setup (config : & Config , stream : & TcpStream) { if config . no_delay { stream . set_nodelay (true) . expect ("Can't set no_delay to true") ; } if config . non_blocking { stream . set_nonblocking (true) . expect ("Can't set channel to be non-blocking") ; } }
};
}
