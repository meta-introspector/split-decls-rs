// Generated macro for impl_122 (impl)
macro_rules! Depcrate_manifest_pathimpl_122 {
() => {
// Module: crate::manifest_path
// Provides: {"impl_122"}
// Dependencies: {}
impl ManifestPath { pub fn parent (& self) -> & AbsPath { self . file . parent () . unwrap () } pub fn canonicalize (& self) -> ! { (* * self) . canonicalize () } pub fn is_rust_manifest (& self) -> bool { self . file . extension () == Some ("rs") } }
};
}
