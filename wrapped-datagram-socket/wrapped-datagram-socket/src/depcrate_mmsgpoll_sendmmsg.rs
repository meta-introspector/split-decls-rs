// Generated macro for poll_sendmmsg (macro)
macro_rules! Depcrate_mmsgpoll_sendmmsg {
() => {
// Module: crate::mmsg
// Provides: {"poll_sendmmsg"}
// Dependencies: {}
# [macro_export] macro_rules ! poll_sendmmsg { ($ self : expr , $ cx : ident , $ bufs : ident) => { loop { match $ self . poll_send_ready ($ cx) ? { Poll :: Ready (()) => { match $ self . try_io (tokio :: io :: Interest :: WRITABLE , || { $ crate :: mmsg :: sendmmsg ($ self . as_fd () , $ bufs) }) { Err (err) if err . kind () == io :: ErrorKind :: WouldBlock => { } res => break Poll :: Ready (res) , } } Poll :: Pending => break Poll :: Pending , } } } ; }
};
}
