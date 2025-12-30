// Generated macro for path_def_id (function)
macro_rules! Depcratepath_def_id {
() => {
// Module: crate
// Provides: {"path_def_id"}
// Dependencies: {}
# [doc = " If `maybe_path` is a path node which resolves to an item, retrieves the item ID"] pub fn path_def_id < 'tcx > (cx : & LateContext < '_ > , maybe_path : & impl MaybePath < 'tcx >) -> Option < DefId > { path_res (cx , maybe_path) . opt_def_id () }
};
}
