// Generated macro for PathResponse (struct)
macro_rules! Depcrate_connection_pathsPathResponse {
() => {
// Module: crate::connection::paths
// Provides: {"PathResponse"}
// Dependencies: {}
# [derive (Copy , Clone)] struct PathResponse { # [doc = " The packet number the corresponding PATH_CHALLENGE was received in"] packet : u64 , token : u64 , # [doc = " The address the corresponding PATH_CHALLENGE was received from"] remote : SocketAddr , }
};
}
