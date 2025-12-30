// Generated macro for impl_444 (impl)
macro_rules! Depcrate_common_rewindimpl_444 {
() => {
// Module: crate::common::rewind
// Provides: {"impl_444"}
// Dependencies: {}
impl < T > Rewind < T > { # [cfg (all (feature = "server" , any (feature = "http1" , feature = "http2")))] pub (crate) fn new_buffered (io : T , buf : Bytes) -> Self { Rewind { pre : Some (buf) , inner : io , } } }
};
}
