// Generated macro for NestedProgress (trait)
macro_rules! Depcrate_traitsNestedProgress {
() => {
// Module: crate::traits
// Provides: {"NestedProgress"}
// Dependencies: {}
# [doc = " A trait for describing hierarchical progress."] pub trait NestedProgress : Progress { # [doc = " The type of progress returned by [`add_child()`][Progress::add_child()]."] type SubProgress : NestedProgress ; # [doc = " Adds a new child, whose parent is this instance, with the given `name`."] # [doc = ""] # [doc = " This will make the child progress to appear contained in the parent progress."] # [doc = " Note that such progress does not have a stable identifier, which can be added"] # [doc = " with [`add_child_with_id()`][Progress::add_child_with_id()] if desired."] fn add_child (& mut self , name : impl Into < String >) -> Self :: SubProgress ; # [doc = " Adds a new child, whose parent is this instance, with the given `name` and `id`."] # [doc = ""] # [doc = " This will make the child progress to appear contained in the parent progress, and it can be identified"] # [doc = " using `id`."] fn add_child_with_id (& mut self , name : impl Into < String > , id : Id) -> Self :: SubProgress ; }
};
}
