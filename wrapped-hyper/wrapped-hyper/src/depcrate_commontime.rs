// Generated macro for time (module)
macro_rules! Depcrate_commontime {
() => {
// Module: crate::common
// Provides: {"time"}
// Dependencies: {}
# [cfg (any (all (feature = "server" , feature = "http1") , all (any (feature = "client" , feature = "server") , feature = "http2") ,))] pub (crate) mod time ;
};
}
