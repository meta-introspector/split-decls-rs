// Generated macro for first_attr_value_str_by_name (function)
macro_rules! Depcrate_attrfirst_attr_value_str_by_name {
() => {
// Module: crate::attr
// Provides: {"first_attr_value_str_by_name"}
// Dependencies: {}
pub fn first_attr_value_str_by_name (attrs : & [impl AttributeExt] , name : Symbol) -> Option < Symbol > { find_by_name (attrs , name) . and_then (| attr | attr . value_str ()) }
};
}
