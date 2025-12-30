// Generated macro for full_name_of_item (function)
macro_rules! Depcrate_clifull_name_of_item {
() => {
// Module: crate::cli
// Provides: {"full_name_of_item"}
// Dependencies: {}
fn full_name_of_item (db : & dyn HirDatabase , module : Module , name : Name) -> String { module . path_to_root (db) . into_iter () . rev () . filter_map (| it | it . name (db)) . chain (Some (name)) . map (| it | it . display (db , Edition :: LATEST) . to_string ()) . join ("::") }
};
}
