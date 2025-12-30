// Generated macro for interface_identifier (function)
macro_rules! Depcrateinterface_identifier {
() => {
// Module: crate
// Provides: {"interface_identifier"}
// Dependencies: {}
fn interface_identifier (interface_id : & WorldKey , resolve : & Resolve , in_export : bool , renamed_interfaces : & HashMap < WorldKey , String > ,) -> String { if let Some (rename) = renamed_interfaces . get (interface_id) { let mut ns = String :: new () ; if in_export && matches ! (interface_id , WorldKey :: Interface (_)) { ns . push_str ("exports_") ; } ns . push_str (rename) ; return ns ; } match interface_id { WorldKey :: Name (name) => name . to_snake_case () , WorldKey :: Interface (id) => { let mut ns = String :: new () ; if in_export { ns . push_str ("exports_") ; } let iface = & resolve . interfaces [* id] ; let pkg = & resolve . packages [iface . package . unwrap ()] ; ns . push_str (& pkg . name . namespace . to_snake_case ()) ; ns . push_str ("_") ; ns . push_str (& pkg . name . name . to_snake_case ()) ; ns . push_str ("_") ; let pkg_has_multiple_versions = resolve . packages . iter () . any (| (_ , p) | { p . name . namespace == pkg . name . namespace && p . name . name == pkg . name . name && p . name . version != pkg . name . version }) ; if pkg_has_multiple_versions { if let Some (version) = & pkg . name . version { let version = version . to_string () . replace ('.' , "_") . replace ('-' , "_") . replace ('+' , "_") ; ns . push_str (& version) ; ns . push_str ("_") ; } } ns . push_str (& iface . name . as_ref () . unwrap () . to_snake_case ()) ; ns } } }
};
}
