// Generated macro for c_func_name (function)
macro_rules! Depcratec_func_name {
() => {
// Module: crate
// Provides: {"c_func_name"}
// Dependencies: {}
pub fn c_func_name (in_import : bool , resolve : & Resolve , world : & str , interface_id : Option < & WorldKey > , func : & Function , renamed_interfaces : & HashMap < WorldKey , String > ,) -> String { let mut name = String :: new () ; match interface_id { Some (id) => name . push_str (& interface_identifier (id , resolve , ! in_import , renamed_interfaces ,)) , None => { if ! in_import { name . push_str ("exports_") ; } name . push_str (& world . to_snake_case ()) ; } } name . push_str ("_") ; name . push_str (& func . name . to_snake_case () . replace ('.' , "_")) ; name }
};
}
