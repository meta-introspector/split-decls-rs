// Generated macro for create_value_object (function)
macro_rules! Depcrate_resolver_utils_containercreate_value_object {
() => {
// Module: crate::resolver_utils::container
// Provides: {"create_value_object"}
// Dependencies: {}
pub (crate) fn create_value_object (values : Vec < (Name , Value) >) -> Value { let mut map = IndexMap :: new () ; for (name , value) in values { insert_value (& mut map , name , value) ; } Value :: Object (map) }
};
}
