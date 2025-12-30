// Generated macro for set_value (function)
macro_rules! Depcrate_sourceset_value {
() => {
// Module: crate::source
// Provides: {"set_value"}
// Dependencies: {}
fn set_value (cache : & mut Value , key : String , value : Value) { match path :: Expression :: from_str (key . as_str ()) { Ok (expr) => expr . set (cache , value) , _ => path :: Expression :: root (key) . set (cache , value) , } }
};
}
