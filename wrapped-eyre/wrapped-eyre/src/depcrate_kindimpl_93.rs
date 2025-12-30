// Generated macro for impl_93 (impl)
macro_rules! Depcrate_kindimpl_93 {
() => {
// Module: crate::kind
// Provides: {"impl_93"}
// Dependencies: {}
impl Adhoc { # [cfg_attr (track_caller , track_caller)] pub fn new < M > (self , message : M) -> Report where M : Display + Debug + Send + Sync + 'static , { Report :: from_adhoc (message) } }
};
}
