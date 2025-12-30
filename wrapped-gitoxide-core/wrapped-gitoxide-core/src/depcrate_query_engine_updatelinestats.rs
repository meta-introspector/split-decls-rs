// Generated macro for LineStats (struct)
macro_rules! Depcrate_query_engine_updateLineStats {
() => {
// Module: crate::query::engine::update
// Provides: {"LineStats"}
// Dependencies: {}
# [doc = " Line statistics for a particular commit."] # [derive (Debug , Default , Copy , Clone)] struct LineStats { # [doc = " amount of added lines"] added : usize , # [doc = " amount of removed lines"] removed : usize , # [doc = " the amount of lines before the change."] before : usize , # [doc = " the amount of lines after the change."] after : usize , }
};
}
