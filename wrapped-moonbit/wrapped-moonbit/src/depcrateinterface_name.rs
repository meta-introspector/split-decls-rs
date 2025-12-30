// Generated macro for interface_name (function)
macro_rules! Depcrateinterface_name {
() => {
// Module: crate
// Provides: {"interface_name"}
// Dependencies: {}
fn interface_name (resolve : & Resolve , name : & WorldKey) -> String { let pkg = match name { WorldKey :: Name (_) => None , WorldKey :: Interface (id) => { let pkg = resolve . interfaces [* id] . package . unwrap () ; Some (resolve . packages [pkg] . name . clone ()) } } ; let name = match name { WorldKey :: Name (name) => name , WorldKey :: Interface (id) => resolve . interfaces [* id] . name . as_ref () . unwrap () , } . to_lower_camel_case () ; format ! ("interface.{}{name}" , if let Some (name) = & pkg { format ! ("{}.{}." , name . namespace . to_moonbit_ident () , name . name . to_moonbit_ident ()) } else { String :: new () }) }
};
}
