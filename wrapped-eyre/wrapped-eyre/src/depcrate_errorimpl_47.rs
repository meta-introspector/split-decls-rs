// Generated macro for impl_47 (impl)
macro_rules! Depcrate_errorimpl_47 {
() => {
// Module: crate::error
// Provides: {"impl_47"}
// Dependencies: {}
impl < E > From < E > for Report where E : StdError + Send + Sync + 'static , { # [cfg_attr (track_caller , track_caller)] fn from (error : E) -> Self { Report :: from_std (error) } }
};
}
