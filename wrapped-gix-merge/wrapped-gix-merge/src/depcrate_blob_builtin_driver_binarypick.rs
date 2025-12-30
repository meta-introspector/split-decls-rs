// Generated macro for Pick (enum)
macro_rules! Depcrate_blob_builtin_driver_binaryPick {
() => {
// Module: crate::blob::builtin_driver::binary
// Provides: {"Pick"}
// Dependencies: {}
# [doc = " Tell the caller of [`merge()`](function::merge) which side was picked."] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum Pick { # [doc = " Chose the ancestor."] Ancestor , # [doc = " Chose our side."] Ours , # [doc = " Chose their side."] Theirs , }
};
}
