// Generated macro for pending (function)
macro_rules! Depcrate_upgradepending {
() => {
// Module: crate::upgrade
// Provides: {"pending"}
// Dependencies: {}
# [cfg (all (any (feature = "client" , feature = "server") , any (feature = "http1" , feature = "http2") ,))] pub (super) fn pending () -> (Pending , OnUpgrade) { let (tx , rx) = oneshot :: channel () ; (Pending { tx } , OnUpgrade { rx : Some (Arc :: new (Mutex :: new (rx))) , } ,) }
};
}
