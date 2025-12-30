// Generated macro for is_optional_arg (function)
macro_rules! Depcrate_options_parseris_optional_arg {
() => {
// Module: crate::options::parser
// Provides: {"is_optional_arg"}
// Dependencies: {}
fn is_optional_arg (value : & OsStr , values : Option < & [& str] >) -> bool { match (values , value . to_str ()) { (Some (values) , Some (value)) => values . contains (& value) , _ => false , } }
};
}
