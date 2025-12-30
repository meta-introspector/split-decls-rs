// Generated macro for Outcome (struct)
macro_rules! Depcrate_cache_delta_traverseOutcome {
() => {
// Module: crate::cache::delta::traverse
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " The outcome of [`Tree::traverse()`]"] pub struct Outcome < T > { # [doc = " The items that have no children in the pack, i.e. base objects."] pub roots : Vec < Item < T > > , # [doc = " The items that children to a root object, i.e. delta objects."] pub children : Vec < Item < T > > , }
};
}
