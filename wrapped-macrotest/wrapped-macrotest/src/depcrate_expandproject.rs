// Generated macro for Project (struct)
macro_rules! Depcrate_expandProject {
() => {
// Module: crate::expand
// Provides: {"Project"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct Project { pub dir : PathBuf , source_dir : PathBuf , # [doc = " Used for the inner runs of cargo()"] pub inner_target_dir : PathBuf , pub name : String , pub features : Option < Vec < String > > , workspace : PathBuf , overwrite : bool , }
};
}
