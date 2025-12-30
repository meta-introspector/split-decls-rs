// Generated macro for namespace (function)
macro_rules! Depcratenamespace {
() => {
// Module: crate
// Provides: {"namespace"}
// Dependencies: {}
fn namespace (resolve : & Resolve , owner : & TypeOwner , guest_export : bool , opts : & Opts) -> Vec < String > { let mut result = Vec :: default () ; if let Some (prefix) = & opts . internal_prefix { result . push (prefix . clone ()) ; } if guest_export { result . push (String :: from ("exports")) ; } match owner { TypeOwner :: World (w) => result . push (resolve . worlds [* w] . name . to_snake_case ()) , TypeOwner :: Interface (i) => { let iface = & resolve . interfaces [* i] ; let pkg = & resolve . packages [iface . package . unwrap ()] ; result . push (pkg . name . namespace . to_snake_case ()) ; result . push (pkg . name . name . to_snake_case ()) ; if let Some (name) = & iface . name { result . push (name . to_snake_case ()) ; } } TypeOwner :: None => () , } result }
};
}
