// Generated macro for TrackRenames (enum)
macro_rules! Depcrate_status_tree_indexTrackRenames {
() => {
// Module: crate::status::tree_index
// Provides: {"TrackRenames"}
// Dependencies: {}
# [doc = " Specify how to perform rewrite tracking [Repository::tree_index_status()]."] # [derive (Default , Debug , Copy , Clone)] pub enum TrackRenames { # [doc = " Check `status.renames` and then `diff.renames` if the former isn't set. Otherwise, default to performing rewrites if nothing"] # [doc = " is set."] # [default] AsConfigured , # [doc = " Track renames according ot the given configuration."] Given (gix_diff :: Rewrites) , # [doc = " Do not track renames."] Disabled , }
};
}
