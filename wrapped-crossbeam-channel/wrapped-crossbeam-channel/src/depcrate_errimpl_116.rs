// Generated macro for impl_116 (impl)
macro_rules! Depcrate_errimpl_116 {
() => {
// Module: crate::err
// Provides: {"impl_116"}
// Dependencies: {}
impl From < RecvError > for TryRecvError { fn from (err : RecvError) -> Self { match err { RecvError => Self :: Disconnected , } } }
};
}
