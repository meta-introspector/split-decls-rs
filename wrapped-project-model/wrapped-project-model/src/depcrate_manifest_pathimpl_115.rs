// Generated macro for impl_115 (impl)
macro_rules! Depcrate_manifest_pathimpl_115 {
() => {
// Module: crate::manifest_path
// Provides: {"impl_115"}
// Dependencies: {}
impl TryFrom < AbsPathBuf > for ManifestPath { type Error = AbsPathBuf ; fn try_from (file : AbsPathBuf) -> Result < Self , Self :: Error > { if file . parent () . is_none () { Err (file) } else { Ok (ManifestPath { file }) } } }
};
}
