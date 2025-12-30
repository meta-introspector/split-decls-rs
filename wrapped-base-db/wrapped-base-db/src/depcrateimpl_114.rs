// Generated macro for impl_114 (impl)
macro_rules! Depcrateimpl_114 {
() => {
// Module: crate
// Provides: {"impl_114"}
// Dependencies: {}
impl CrateWorkspaceData { pub fn is_atleast_187 (& self) -> bool { const VERSION_187 : Version = Version { major : 1 , minor : 87 , patch : 0 , pre : Prerelease :: EMPTY , build : BuildMetadata :: EMPTY , } ; self . toolchain . as_ref () . map_or (false , | v | * v >= VERSION_187) } }
};
}
