// Generated macro for network_run (function)
macro_rules! Depcrate_executor_networknetwork_run {
() => {
// Module: crate::executor::network
// Provides: {"network_run"}
// Dependencies: {}
async fn network_run () { future :: poll_fn (| cx | { if let Some (mut guard) = NIC . try_lock () { match & mut * guard { NetworkState :: Initialized (nic) => { nic . poll_common (now ()) ; cx . waker () . wake_by_ref () ; Poll :: Pending } _ => Poll :: Ready (()) , } } else { cx . waker () . wake_by_ref () ; Poll :: Pending } }) . await ; }
};
}
