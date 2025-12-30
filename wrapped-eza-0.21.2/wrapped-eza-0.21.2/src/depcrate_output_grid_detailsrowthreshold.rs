// Generated macro for RowThreshold (enum)
macro_rules! Depcrate_output_grid_detailsRowThreshold {
() => {
// Module: crate::output::grid_details
// Provides: {"RowThreshold"}
// Dependencies: {}
# [doc = " The grid-details view can be configured to revert to just a details view"] # [doc = " (with one column) if it wouldn’t produce enough rows of output."] # [doc = ""] # [doc = " Doing this makes the resulting output look a bit better: when listing a"] # [doc = " small directory of four files in four columns, the files just look spaced"] # [doc = " out and it’s harder to see what’s going on. So it can be enabled just for"] # [doc = " larger directory listings."] # [derive (PartialEq , Eq , Debug , Copy , Clone)] pub enum RowThreshold { # [doc = " Only use grid-details view if it would result in at least this many"] # [doc = " rows of output."] MinimumRows (usize) , # [doc = " Use the grid-details view no matter what."] AlwaysGrid , }
};
}
