// Generated macro for impl_105 (impl)
macro_rules! Depcrate_errimpl_105 {
() => {
// Module: crate::err
// Provides: {"impl_105"}
// Dependencies: {}
impl < T > From < SendError < T > > for TrySendError < T > { fn from (err : SendError < T >) -> Self { match err { SendError (t) => Self :: Disconnected (t) , } } }
};
}
