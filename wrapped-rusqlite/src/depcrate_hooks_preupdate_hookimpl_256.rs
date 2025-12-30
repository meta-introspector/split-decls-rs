// Generated macro for impl_256 (impl)
macro_rules! Depcrate_hooks_preupdate_hookimpl_256 {
() => {
// Module: crate::hooks::preupdate_hook
// Provides: {"impl_256"}
// Dependencies: {}
impl From < PreUpdateCase > for Action { fn from (puc : PreUpdateCase) -> Action { match puc { PreUpdateCase :: Insert (_) => Action :: SQLITE_INSERT , PreUpdateCase :: Delete (_) => Action :: SQLITE_DELETE , PreUpdateCase :: Update { .. } => Action :: SQLITE_UPDATE , PreUpdateCase :: Unknown => Action :: UNKNOWN , } } }
};
}
