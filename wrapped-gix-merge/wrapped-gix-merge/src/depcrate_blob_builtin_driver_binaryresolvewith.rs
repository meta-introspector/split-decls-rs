// Generated macro for ResolveWith (enum)
macro_rules! Depcrate_blob_builtin_driver_binaryResolveWith {
() => {
// Module: crate::blob::builtin_driver::binary
// Provides: {"ResolveWith"}
// Dependencies: {}
# [doc = " What to do when having to pick a side to resolve a conflict."] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum ResolveWith { # [doc = " Chose the ancestor to resolve a conflict."] Ancestor , # [doc = " Chose our side to resolve a conflict."] Ours , # [doc = " Chose their side to resolve a conflict."] Theirs , }
};
}
