// Generated macro for impl_514 (impl)
macro_rules! Depcrate_syncimpl_514 {
() => {
// Module: crate::sync
// Provides: {"impl_514"}
// Dependencies: {}
# [doc (hidden)] impl < A > Future for SyncArbiter < A > where A : Actor < Context = SyncContext < A > > , { type Output = () ; fn poll (self : Pin < & mut Self > , cx : & mut task :: Context < '_ >) -> Poll < Self :: Output > { let this = self . get_mut () ; loop { match Pin :: new (& mut this . msgs) . poll_next (cx) { Poll :: Ready (Some (msg)) => { if let Some (ref queue) = this . queue { assert ! (queue . send (msg) . is_ok ()) ; } } Poll :: Pending => break , Poll :: Ready (None) => unreachable ! () , } } if this . msgs . connected () { Poll :: Pending } else { this . queue = None ; Poll :: Ready (()) } } }
};
}
