// Generated macro for get_or_insert_recv (function)
macro_rules! Depcrate_connection_streams_stateget_or_insert_recv {
() => {
// Module: crate::connection::streams::state
// Provides: {"get_or_insert_recv"}
// Dependencies: {}
# [inline] pub (super) fn get_or_insert_recv (initial_max_data : u64 ,) -> impl FnMut (& mut Option < StreamRecv >) -> & mut Recv { move | opt | { * opt = opt . take () . map (| s | match s { StreamRecv :: Free (recv) => StreamRecv :: Open (recv) , s => s , }) ; opt . get_or_insert_with (| | StreamRecv :: Open (Recv :: new (initial_max_data))) . as_open_recv_mut () . unwrap () } }
};
}
