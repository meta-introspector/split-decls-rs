// Generated macro for imported_types_used_by_exported_interfaces (function)
macro_rules! Depcrateimported_types_used_by_exported_interfaces {
() => {
// Module: crate
// Provides: {"imported_types_used_by_exported_interfaces"}
// Dependencies: {}
pub fn imported_types_used_by_exported_interfaces (resolve : & Resolve , world : WorldId ,) -> HashSet < TypeId > { let mut live_export_types = LiveTypes :: default () ; let mut exported_interfaces = HashSet :: new () ; for (_ , export) in resolve . worlds [world] . exports . iter () { match export { WorldItem :: Function (_) => { } WorldItem :: Interface { id , .. } => { exported_interfaces . insert (* id) ; live_export_types . add_interface (resolve , * id) } WorldItem :: Type (_) => unreachable ! () , } } let mut imports_used = HashSet :: new () ; for ty in live_export_types . iter () { if let TypeOwner :: Interface (id) = resolve . types [ty] . owner { if ! exported_interfaces . contains (& id) { imports_used . insert (id) ; } } } let mut live_import_types = LiveTypes :: default () ; for import in imports_used { live_import_types . add_interface (resolve , import) ; } let live_import_types = live_import_types . iter () . collect :: < HashSet < _ > > () ; live_import_types }
};
}
