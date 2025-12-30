// Generated macro for extract_crate_names (function)
macro_rules! Depcrateextract_crate_names {
() => {
// Module: crate
// Provides: {"extract_crate_names"}
// Dependencies: {}
# [doc = " Extract all crate names from the given `Cargo.toml` by checking the `dependencies` and"] # [doc = " `dev-dependencies`."] fn extract_crate_names (cargo_toml : & DocumentMut , workspace_dependencies : BTreeMap < String , String > ,) -> Result < CrateNames , Error > { let package_name = extract_package_name (cargo_toml) ; let root_pkg = package_name . as_ref () . map (| name | { let cr = match env :: var_os ("CARGO_TARGET_TMPDIR") { None => FoundCrate :: Itself , Some (_) => FoundCrate :: Name (sanitize_crate_name (name)) , } ; (name . to_string () , cr) }) ; let dep_tables = dep_tables (cargo_toml . as_table ()) . chain (target_dep_tables (cargo_toml)) ; let dep_pkgs = dep_tables . map (| t | t . iter ()) . flatten () . filter_map (move | (dep_name , dep_value) | { let pkg_name = dep_value . get ("package") . and_then (| i | i . as_str ()) . unwrap_or (dep_name) ; if package_name . as_ref () . map_or (false , | n | * n == pkg_name) { return None } let workspace = dep_value . get ("workspace") . and_then (| w | w . as_bool ()) . unwrap_or_default () ; let pkg_name = workspace . then (| | workspace_dependencies . get (pkg_name) . map (| p | p . as_ref ())) . flatten () . unwrap_or (pkg_name) ; let cr = FoundCrate :: Name (sanitize_crate_name (dep_name)) ; Some ((pkg_name . to_owned () , cr)) }) ; Ok (root_pkg . into_iter () . chain (dep_pkgs) . collect ()) }
};
}
