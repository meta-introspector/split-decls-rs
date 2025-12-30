// Generated macro for error (module)
macro_rules! Depcrate_types_utilitieserror {
() => {
// Module: crate::types::utilities
// Provides: {"error"}
// Dependencies: {}
# [doc = " Common error messages used in validation and execution of GraphQL operations"] pub (crate) mod error { use std :: fmt :: Display ; pub (crate) fn non_null (arg_type : impl Display) -> String { format ! ("\"null\" specified for not nullable type \"{arg_type}\"") } pub (crate) fn enum_value (arg_value : impl Display , arg_type : impl Display) -> String { format ! ("Invalid value \"{arg_value}\" for enum \"{arg_type}\"") } pub (crate) fn type_value (arg_value : impl Display , arg_type : impl Display) -> String { format ! ("Invalid value \"{arg_value}\" for type \"{arg_type}\"") } pub (crate) fn parser (arg_type : impl Display , msg : impl Display) -> String { format ! ("Parser error for \"{arg_type}\": {msg}") } pub (crate) fn not_input_object (arg_type : impl Display) -> String { format ! ("\"{arg_type}\" is not an input object") } pub (crate) fn field (arg_type : impl Display , field_name : impl Display , error_message : impl Display ,) -> String { format ! ("Error on \"{arg_type}\" field \"{field_name}\": {error_message}") } pub (crate) fn missing_fields (arg_type : impl Display , missing_fields : impl Display) -> String { format ! ("\"{arg_type}\" is missing fields: {missing_fields}") } pub (crate) fn unknown_field (arg_type : impl Display , field_name : impl Display) -> String { format ! ("Field \"{field_name}\" does not exist on type \"{arg_type}\"") } pub (crate) fn invalid_list_length (arg_value : impl Display , actual : usize , expected : usize ,) -> String { format ! ("Expected list of length {expected}, but \"{arg_value}\" has length {actual}") } }
};
}
