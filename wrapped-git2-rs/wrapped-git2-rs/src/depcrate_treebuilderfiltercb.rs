// Generated macro for FilterCb (type)
macro_rules! Depcrate_treebuilderFilterCb {
() => {
// Module: crate::treebuilder
// Provides: {"FilterCb"}
// Dependencies: {}
type FilterCb < 'a > = dyn FnMut (& TreeEntry < '_ >) -> bool + 'a ;
};
}
