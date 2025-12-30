// Generated macro for target_dep_tables (function)
macro_rules! Depcratetarget_dep_tables {
() => {
// Module: crate
// Provides: {"target_dep_tables"}
// Dependencies: {}
fn target_dep_tables (cargo_toml : & DocumentMut) -> impl Iterator < Item = & dyn TableLike > { cargo_toml . get ("target") . into_iter () . filter_map (Item :: as_table_like) . flat_map (| t | { t . iter () . map (| (_ , value) | value) . filter_map (Item :: as_table_like) . flat_map (dep_tables) }) }
};
}
