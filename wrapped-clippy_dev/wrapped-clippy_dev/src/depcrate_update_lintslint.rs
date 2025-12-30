// Generated macro for Lint (struct)
macro_rules! Depcrate_update_lintsLint {
() => {
// Module: crate::update_lints
// Provides: {"Lint"}
// Dependencies: {}
# [doc = " Lint data parsed from the Clippy source code."] # [derive (PartialEq , Eq , Debug)] pub struct Lint { pub name : String , pub group : String , pub module : String , pub path : PathBuf , pub declaration_range : Range < usize > , }
};
}
