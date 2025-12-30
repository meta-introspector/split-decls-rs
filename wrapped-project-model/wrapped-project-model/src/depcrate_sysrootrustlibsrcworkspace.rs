// Generated macro for RustLibSrcWorkspace (enum)
macro_rules! Depcrate_sysrootRustLibSrcWorkspace {
() => {
// Module: crate::sysroot
// Provides: {"RustLibSrcWorkspace"}
// Dependencies: {}
# [derive (Debug , Clone , Eq , PartialEq)] pub enum RustLibSrcWorkspace { Workspace { ws : CargoWorkspace , metadata_err : Option < String > } , Json (ProjectJson) , Stitched (stitched :: Stitched) , Empty , }
};
}
