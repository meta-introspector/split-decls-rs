// Generated macro for dep_tables (function)
macro_rules! Depcratedep_tables {
() => {
// Module: crate
// Provides: {"dep_tables"}
// Dependencies: {}
fn dep_tables (table : & dyn TableLike) -> impl Iterator < Item = & dyn TableLike > { table . get ("dependencies") . into_iter () . chain (table . get ("dev-dependencies")) . filter_map (Item :: as_table_like) }
};
}
