// Generated macro for ChangeLocation (enum)
macro_rules! Depcrate_tree_utilsChangeLocation {
() => {
// Module: crate::tree::utils
// Provides: {"ChangeLocation"}
// Dependencies: {}
# [derive (Debug , Default , Clone , Copy)] enum ChangeLocation { # [doc = " The change is at its current (and only) location, or in the source location of a rename."] # [default] CurrentLocation , # [doc = " This is always the destination of a rename."] RenamedLocation , }
};
}
