// Generated macro for get_manifest (function)
macro_rules! Depcrate_dependenciesget_manifest {
() => {
// Module: crate::dependencies
// Provides: {"get_manifest"}
// Dependencies: {}
pub (crate) fn get_manifest (manifest_dir : & Path) -> Manifest { try_get_manifest (manifest_dir) . unwrap_or_default () }
};
}
