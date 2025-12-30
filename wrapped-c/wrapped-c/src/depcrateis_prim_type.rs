// Generated macro for is_prim_type (function)
macro_rules! Depcrateis_prim_type {
() => {
// Module: crate
// Provides: {"is_prim_type"}
// Dependencies: {}
fn is_prim_type (resolve : & Resolve , ty : & Type) -> bool { if let Type :: Id (id) = ty { is_prim_type_id (resolve , * id) } else { true } }
};
}
