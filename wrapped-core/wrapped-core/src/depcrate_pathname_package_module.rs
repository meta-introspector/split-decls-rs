// Generated macro for name_package_module (function)
macro_rules! Depcrate_pathname_package_module {
() => {
// Module: crate::path
// Provides: {"name_package_module"}
// Dependencies: {}
# [doc = " If the package `id` is the only package with its namespace/name combo"] # [doc = " then pass through the name unmodified. If, however, there are multiple"] # [doc = " versions of this package then the package module is going to get version"] # [doc = " information."] pub fn name_package_module (resolve : & Resolve , id : PackageId) -> String { let pkg = & resolve . packages [id] ; let versions_with_same_name = resolve . packages . iter () . filter_map (| (_ , p) | { if p . name . namespace == pkg . name . namespace && p . name . name == pkg . name . name { Some (& p . name . version) } else { None } }) . collect :: < Vec < _ > > () ; let base = pkg . name . name . to_snake_case () ; if versions_with_same_name . len () == 1 { return base ; } let version = match & pkg . name . version { Some (version) => version , None => return base , } ; let version = version . to_string () . replace ('.' , "_") . replace ('-' , "_") . replace ('+' , "_") . to_snake_case () ; format ! ("{base}{version}") }
};
}
