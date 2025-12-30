// Generated macro for owner_namespace (function)
macro_rules! Depcrateowner_namespace {
() => {
// Module: crate
// Provides: {"owner_namespace"}
// Dependencies: {}
pub fn owner_namespace < 'a > (interface : Option < (InterfaceId , & 'a WorldKey) > , in_import : bool , world : String , resolve : & Resolve , id : TypeId , renamed_interfaces : & HashMap < WorldKey , String > ,) -> String { let ty = & resolve . types [id] ; match (ty . owner , interface) { (TypeOwner :: Interface (a) , Some ((b , key))) if a == b => { interface_identifier (key , resolve , ! in_import , renamed_interfaces) } (TypeOwner :: Interface (_) , None) => unreachable ! () , (TypeOwner :: Interface (_) , Some (_)) => unreachable ! () , (TypeOwner :: World (_) , None) => world . to_snake_case () , (TypeOwner :: World (_) , Some (_)) => unreachable ! () , (TypeOwner :: None , Some ((_ , key))) => { interface_identifier (key , resolve , ! in_import , renamed_interfaces) } (TypeOwner :: None , None) => world . to_snake_case () , } }
};
}
