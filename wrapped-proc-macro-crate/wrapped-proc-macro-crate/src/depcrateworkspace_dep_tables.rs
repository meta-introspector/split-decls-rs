// Generated macro for workspace_dep_tables (function)
macro_rules! Depcrateworkspace_dep_tables {
() => {
// Module: crate
// Provides: {"workspace_dep_tables"}
// Dependencies: {}
# [doc = " Return an iterator over all `[workspace.dependencies]`"] fn workspace_dep_tables (cargo_toml : & DocumentMut) -> Option < & dyn TableLike > { cargo_toml . get ("workspace") . and_then (| w | w . as_table_like () ? . get ("dependencies") ? . as_table_like ()) }
};
}
