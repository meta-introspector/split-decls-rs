// Generated macro for future (module)
macro_rules! Depcrate_commonfuture {
() => {
// Module: crate::common
// Provides: {"future"}
// Dependencies: {}
# [cfg (any (all (feature = "client" , any (feature = "http1" , feature = "http2")) , all (feature = "server" , feature = "http1") ,))] pub (crate) mod future ;
};
}
