// Generated macro for Delegate (trait)
macro_rules! Depcrate_spec_parseDelegate {
() => {
// Module: crate::spec::parse
// Provides: {"Delegate"}
// Dependencies: {}
# [doc = " A delegate to be informed about parse events, with methods split into categories."] # [doc = ""] # [doc = " - **Anchors** - which revision to use as starting point for…"] # [doc = " - **Navigation** - where to go once from the initial revision"] # [doc = " - **Range** - to learn if the specification is for a single or multiple references, and how to combine them."] pub trait Delegate : delegate :: Revision + delegate :: Navigate + delegate :: Kind { # [doc = " Called at the end of a successful parsing operation."] # [doc = " It can be used as a marker to finalize internal data structures."] # [doc = ""] # [doc = " Note that it will not be called if there is unconsumed input."] fn done (& mut self) ; }
};
}
