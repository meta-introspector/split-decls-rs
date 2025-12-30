// Generated macro for get_type_name (function)
macro_rules! Depcrate_deriveget_type_name {
() => {
// Module: crate::derive
// Provides: {"get_type_name"}
// Dependencies: {}
fn get_type_name (reader : & Reader , path : & str) -> TypeName { if let Some ((namespace , name)) = path . rsplit_once ('.') { if let Some ((namespace , types)) = reader . get_key_value (namespace) { if let Some ((name , _)) = types . get_key_value (name) { return TypeName (namespace , name) ; } } } else { for (namespace , types) in reader . iter () { if let Some ((name , _)) = types . get_key_value (path) { return TypeName (namespace , name) ; } } } panic ! ("type not found: `{path}`") ; }
};
}
