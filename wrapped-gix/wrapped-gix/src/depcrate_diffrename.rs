// Generated macro for rename (module)
macro_rules! Depcrate_diffrename {
() => {
// Module: crate::diff
// Provides: {"rename"}
// Dependencies: {}
# [doc = ""] pub mod rename { # [doc = " Determine how to do rename tracking."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum Tracking { # [doc = " Do not track renames at all, the fastest option."] Disabled , # [doc = " Track renames."] Renames , # [doc = " Track renames and copies."] # [doc = ""] # [doc = " This is the most expensive option."] RenamesAndCopies , } }
};
}
