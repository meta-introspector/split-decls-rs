// Generated macro for PathMap (struct)
macro_rules! Depcrate_pathPathMap {
() => {
// Module: crate::path
// Provides: {"PathMap"}
// Dependencies: {}
# [doc = " All path-related information."] pub struct PathMap { # [doc = " The paths of the connection. Each of them has an internal identifier"] # [doc = " that is used by `addrs_to_paths` and `ConnectionEntry`."] paths : Slab < Path > , # [doc = " The maximum number of concurrent paths allowed."] max_concurrent_paths : usize , # [doc = " The mapping from the (local `SocketAddr`, peer `SocketAddr`) to the"] # [doc = " `Path` structure identifier."] addrs_to_paths : BTreeMap < (SocketAddr , SocketAddr) , usize > , # [doc = " Path-specific events to be notified to the application."] events : VecDeque < PathEvent > , # [doc = " Whether this manager serves a connection as a server."] is_server : bool , }
};
}
