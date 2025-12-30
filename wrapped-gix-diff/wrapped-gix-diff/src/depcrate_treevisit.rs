// Generated macro for Visit (trait)
macro_rules! Depcrate_treeVisit {
() => {
// Module: crate::tree
// Provides: {"Visit"}
// Dependencies: {}
# [doc = " A trait to allow responding to a traversal designed to figure out the [changes](visit::Change)"] # [doc = " to turn tree A into tree B."] pub trait Visit { # [doc = " Sets the full path in front of the queue so future calls to push and pop components affect it instead."] fn pop_front_tracked_path_and_set_current (& mut self) ; # [doc = " Append a `component` to the end of a path, which may be empty."] fn push_back_tracked_path_component (& mut self , component : & BStr) ; # [doc = " Append a `component` to the end of a path, which may be empty."] fn push_path_component (& mut self , component : & BStr) ; # [doc = " Removes the last component from the path, which may leave it empty."] fn pop_path_component (& mut self) ; # [doc = " Record a `change` and return an instruction whether to continue or not."] # [doc = ""] # [doc = " The implementation may use the current path to lean where in the tree the change is located."] fn visit (& mut self , change : visit :: Change) -> visit :: Action ; }
};
}
