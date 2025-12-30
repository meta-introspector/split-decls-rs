// Generated macro for is_path_lang_item (function)
macro_rules! Depcrateis_path_lang_item {
() => {
// Module: crate
// Provides: {"is_path_lang_item"}
// Dependencies: {}
# [doc = " If `maybe_path` is a path node which resolves to an item, resolves it to a `DefId` and checks if"] # [doc = " it matches the given lang item."] pub fn is_path_lang_item < 'tcx > (cx : & LateContext < '_ > , maybe_path : & impl MaybePath < 'tcx > , lang_item : LangItem) -> bool { path_def_id (cx , maybe_path) . is_some_and (| id | cx . tcx . lang_items () . get (lang_item) == Some (id)) }
};
}
