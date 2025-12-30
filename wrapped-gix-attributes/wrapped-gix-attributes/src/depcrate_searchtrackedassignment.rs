// Generated macro for TrackedAssignment (struct)
macro_rules! Depcrate_searchTrackedAssignment {
() => {
// Module: crate::search
// Provides: {"TrackedAssignment"}
// Dependencies: {}
# [doc = " A way to have an assignment (`attr=value`) but also associated it with an id that allows perfect mapping"] # [doc = " to tracking information."] # [doc = " Note that the order is produced after the files are parsed as global ordering is needed that goes beyond the scope of a"] # [doc = " single `Search` instance."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub struct TrackedAssignment { # [doc = " The order of the assignment."] pub id : AttributeId , # [doc = " The actual assignment information."] pub inner : Assignment , }
};
}
