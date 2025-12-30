// Generated macro for is_path_diagnostic_item (function)
macro_rules! Depcrateis_path_diagnostic_item {
() => {
// Module: crate
// Provides: {"is_path_diagnostic_item"}
// Dependencies: {}
# [doc = " If `maybe_path` is a path node which resolves to an item, resolves it to a `DefId` and checks if"] # [doc = " it matches the given diagnostic item."] pub fn is_path_diagnostic_item < 'tcx > (cx : & LateContext < '_ > , maybe_path : & impl MaybePath < 'tcx > , diag_item : Symbol ,) -> bool { path_def_id (cx , maybe_path) . is_some_and (| id | cx . tcx . is_diagnostic_item (diag_item , id)) }
};
}
