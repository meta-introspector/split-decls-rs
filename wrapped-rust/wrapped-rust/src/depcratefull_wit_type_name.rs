// Generated macro for full_wit_type_name (function)
macro_rules! Depcratefull_wit_type_name {
() => {
// Module: crate
// Provides: {"full_wit_type_name"}
// Dependencies: {}
# [doc = " Returns the full WIT type name with fully qualified interface name"] fn full_wit_type_name (resolve : & Resolve , id : TypeId) -> String { let id = dealias (resolve , id) ; let type_def = & resolve . types [id] ; let interface_name = match type_def . owner { TypeOwner :: World (w) => Some (resolve . worlds [w] . name . clone ()) , TypeOwner :: Interface (id) => resolve . id_of (id) , TypeOwner :: None => None , } ; match interface_name { Some (interface_name) => format ! ("{}/{}" , interface_name , type_def . name . clone () . unwrap ()) , None => type_def . name . clone () . unwrap () , } }
};
}
