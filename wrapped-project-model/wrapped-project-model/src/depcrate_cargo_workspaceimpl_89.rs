// Generated macro for impl_89 (impl)
macro_rules! Depcrate_cargo_workspaceimpl_89 {
() => {
// Module: crate::cargo_workspace
// Provides: {"impl_89"}
// Dependencies: {}
impl TargetDirectoryConfig { pub fn target_dir < 'a > (& 'a self , ws_target_dir : Option < & 'a Utf8Path > ,) -> Option < Cow < 'a , Utf8Path > > { match self { TargetDirectoryConfig :: None => None , TargetDirectoryConfig :: UseSubdirectory => { Some (Cow :: Owned (ws_target_dir ? . join ("rust-analyzer"))) } TargetDirectoryConfig :: Directory (dir) => Some (Cow :: Borrowed (dir)) , } } }
};
}
