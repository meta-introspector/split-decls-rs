// Generated macro for impl_124 (impl)
macro_rules! Depcrate_recv_streamimpl_124 {
() => {
// Module: crate::recv_stream
// Provides: {"impl_124"}
// Dependencies: {}
impl < T > From < (Option < T > , Option < proto :: ReadError >) > for ReadStatus < T > { fn from (status : (Option < T > , Option < proto :: ReadError >)) -> Self { match status { (read , None) => Self :: Finished (read) , (read , Some (e)) => Self :: Failed (read , e) , } } }
};
}
