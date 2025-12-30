// Generated macro for QueryPathNode (struct)
macro_rules! Depcrate_contextQueryPathNode {
() => {
// Module: crate::context
// Provides: {"QueryPathNode"}
// Dependencies: {}
# [doc = " A path to the current query."] # [doc = ""] # [doc = " The path is stored as a kind of reverse linked list."] # [derive (Debug , Clone , Copy)] pub struct QueryPathNode < 'a > { # [doc = " The parent node to this, if there is one."] pub parent : Option < & 'a QueryPathNode < 'a > > , # [doc = " The current path segment being resolved."] pub segment : QueryPathSegment < 'a > , }
};
}
