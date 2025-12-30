// Generated macro for extract_workspace_dependencies (function)
macro_rules! Depcrateextract_workspace_dependencies {
() => {
// Module: crate
// Provides: {"extract_workspace_dependencies"}
// Dependencies: {}
# [doc = " Extract all `[workspace.dependencies]`."] # [doc = ""] # [doc = " Returns a hash map that maps from dep name to the package name. Dep name"] # [doc = " and package name can be the same if there doesn't exist any rename."] fn extract_workspace_dependencies (workspace_toml : & DocumentMut ,) -> Result < BTreeMap < String , String > , Error > { Ok (workspace_dep_tables (& workspace_toml) . into_iter () . map (| t | t . iter ()) . flatten () . map (move | (dep_name , dep_value) | { let pkg_name = dep_value . get ("package") . and_then (| i | i . as_str ()) . unwrap_or (dep_name) ; (dep_name . to_owned () , pkg_name . to_owned ()) }) . collect ()) }
};
}
