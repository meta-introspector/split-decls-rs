// Generated macro for TreeWalkCb (type)
macro_rules! Depcrate_treeTreeWalkCb {
() => {
// Module: crate::tree
// Provides: {"TreeWalkCb"}
// Dependencies: {}
type TreeWalkCb < 'a , T > = dyn FnMut (& str , & TreeEntry < '_ >) -> T + 'a ;
};
}
