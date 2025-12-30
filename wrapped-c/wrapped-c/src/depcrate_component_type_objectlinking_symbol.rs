// Generated macro for linking_symbol (function)
macro_rules! Depcrate_component_type_objectlinking_symbol {
() => {
// Module: crate::component_type_object
// Provides: {"linking_symbol"}
// Dependencies: {}
pub fn linking_symbol (name : & str) -> String { let snake = name . to_snake_case () ; format ! ("__component_type_object_force_link_{snake}") }
};
}
