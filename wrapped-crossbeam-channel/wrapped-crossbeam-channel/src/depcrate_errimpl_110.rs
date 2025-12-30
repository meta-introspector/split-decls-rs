// Generated macro for impl_110 (impl)
macro_rules! Depcrate_errimpl_110 {
() => {
// Module: crate::err
// Provides: {"impl_110"}
// Dependencies: {}
impl < T > From < SendError < T > > for SendTimeoutError < T > { fn from (err : SendError < T >) -> Self { match err { SendError (e) => Self :: Disconnected (e) , } } }
};
}
