// Generated macro for error_container_inside_struct (function)
macro_rules! Depcrate_serializererror_container_inside_struct {
() => {
// Module: crate::serializer
// Provides: {"error_container_inside_struct"}
// Dependencies: {}
fn error_container_inside_struct < T : fmt :: Display > (name : T) -> Error { Error :: custom (format ! ("cannot serialize {} container inside struct \
         when writing headers from structs" , name)) }
};
}
