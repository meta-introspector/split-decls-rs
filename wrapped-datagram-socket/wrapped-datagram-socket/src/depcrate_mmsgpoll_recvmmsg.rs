// Generated macro for poll_recvmmsg (macro)
macro_rules! Depcrate_mmsgpoll_recvmmsg {
() => {
// Module: crate::mmsg
// Provides: {"poll_recvmmsg"}
// Dependencies: {}
# [macro_export] macro_rules ! poll_recvmmsg { ($ self : expr , $ cx : ident , $ bufs : ident) => { loop { match $ self . poll_recv_ready ($ cx) ? { Poll :: Ready (()) => { match $ self . try_io (tokio :: io :: Interest :: READABLE , || { $ crate :: mmsg :: recvmmsg ($ self . as_fd () , $ bufs) }) { Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => { } res => break Poll :: Ready (res) , } } Poll :: Pending => break Poll :: Pending , } } } ; }
};
}
