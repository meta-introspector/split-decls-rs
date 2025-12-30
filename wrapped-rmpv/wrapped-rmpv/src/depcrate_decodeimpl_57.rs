// Generated macro for impl_57 (impl)
macro_rules! Depcrate_decodeimpl_57 {
() => {
// Module: crate::decode
// Provides: {"impl_57"}
// Dependencies: {}
impl From < Error > for io :: Error { # [cold] fn from (val : Error) -> Self { match val { Error :: InvalidMarkerRead (err) | Error :: InvalidDataRead (err) => err , Error :: DepthLimitExceeded => Self :: new (val . kind () , val) , } } }
};
}
