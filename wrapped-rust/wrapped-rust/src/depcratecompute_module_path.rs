// Generated macro for compute_module_path (function)
macro_rules! Depcratecompute_module_path {
() => {
// Module: crate
// Provides: {"compute_module_path"}
// Dependencies: {}
fn compute_module_path (name : & WorldKey , resolve : & Resolve , is_export : bool) -> Vec < String > { let mut path = Vec :: new () ; if is_export { path . push ("exports" . to_string ()) ; } match name { WorldKey :: Name (name) => { path . push (to_rust_ident (name)) ; } WorldKey :: Interface (id) => { let iface = & resolve . interfaces [* id] ; let pkg = iface . package . unwrap () ; let pkgname = resolve . packages [pkg] . name . clone () ; path . push (to_rust_ident (& pkgname . namespace)) ; path . push (name_package_module (resolve , pkg)) ; path . push (to_rust_ident (iface . name . as_ref () . unwrap ())) ; } } path }
};
}
