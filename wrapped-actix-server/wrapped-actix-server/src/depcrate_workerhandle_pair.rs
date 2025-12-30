// Generated macro for handle_pair (function)
macro_rules! Depcrate_workerhandle_pair {
() => {
// Module: crate::worker
// Provides: {"handle_pair"}
// Dependencies: {}
# [doc = " Create accept and server worker handles."] fn handle_pair (idx : usize , conn_tx : UnboundedSender < Conn > , stop_tx : UnboundedSender < Stop > , counter : Counter ,) -> (WorkerHandleAccept , WorkerHandleServer) { let accept = WorkerHandleAccept { idx , conn_tx , counter , } ; let server = WorkerHandleServer { idx , stop_tx } ; (accept , server) }
};
}
