// Generated macro for TargetData (struct)
macro_rules! Depcrate_cargo_workspaceTargetData {
() => {
// Module: crate::cargo_workspace
// Provides: {"TargetData"}
// Dependencies: {}
# [doc = " Information associated with a package's target"] # [derive (Debug , Clone , Eq , PartialEq)] pub struct TargetData { # [doc = " Package that provided this target"] pub package : Package , # [doc = " Name as given in the `Cargo.toml` or generated from the file name"] pub name : String , # [doc = " Path to the main source file of the target"] pub root : AbsPathBuf , # [doc = " Kind of target"] pub kind : TargetKind , # [doc = " Required features of the target without which it won't build"] pub required_features : Vec < String > , }
};
}
