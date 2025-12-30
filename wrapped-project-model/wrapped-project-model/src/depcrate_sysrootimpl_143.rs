// Generated macro for impl_143 (impl)
macro_rules! Depcrate_sysrootimpl_143 {
() => {
// Module: crate::sysroot
// Provides: {"impl_143"}
// Dependencies: {}
impl fmt :: Display for RustLibSrcWorkspace { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { RustLibSrcWorkspace :: Workspace { ws , .. } => { write ! (f , "workspace {}" , ws . workspace_root ()) } RustLibSrcWorkspace :: Json (json) => write ! (f , "json {}" , json . manifest_or_root ()) , RustLibSrcWorkspace :: Stitched (stitched) => { write ! (f , "stitched with {} crates" , stitched . crates . len ()) } RustLibSrcWorkspace :: Empty => write ! (f , "empty") , } } }
};
}
