// Generated macro for impl_120 (impl)
macro_rules! Depcrate_errimpl_120 {
() => {
// Module: crate::err
// Provides: {"impl_120"}
// Dependencies: {}
impl From < RecvError > for RecvTimeoutError { fn from (err : RecvError) -> Self { match err { RecvError => Self :: Disconnected , } } }
};
}
