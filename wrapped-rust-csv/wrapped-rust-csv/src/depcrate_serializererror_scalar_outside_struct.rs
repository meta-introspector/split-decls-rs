// Generated macro for error_scalar_outside_struct (function)
macro_rules! Depcrate_serializererror_scalar_outside_struct {
() => {
// Module: crate::serializer
// Provides: {"error_scalar_outside_struct"}
// Dependencies: {}
fn error_scalar_outside_struct < T : fmt :: Display > (name : T) -> Error { Error :: custom (format ! ("cannot serialize {} scalar outside struct \
         when writing headers from structs" , name)) }
};
}
