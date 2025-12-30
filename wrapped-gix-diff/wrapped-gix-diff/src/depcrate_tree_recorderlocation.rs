// Generated macro for Location (enum)
macro_rules! Depcrate_tree_recorderLocation {
() => {
// Module: crate::tree::recorder
// Provides: {"Location"}
// Dependencies: {}
# [doc = " Describe how to track the location of a change."] # [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum Location { # [doc = " Track the entire path, relative to the repository."] Path , # [doc = " Keep only the file-name as location, which may be enough for some calculations."] # [doc = ""] # [doc = " This is less expensive than tracking the entire `Path`."] FileName , }
};
}
