// Generated macro for impl_643 (impl)
macro_rules! Depcrate_ir_entitiesimpl_643 {
() => {
// Module: crate::ir::entities
// Provides: {"impl_643"}
// Dependencies: {}
impl Value { # [doc = " Create a value from its number representation."] # [doc = " This is the number in the `vNN` notation."] # [doc = ""] # [doc = " This method is for use by the parser."] pub fn with_number (n : u32) -> Option < Self > { if n < u32 :: MAX / 2 { Some (Self (n)) } else { None } } }
};
}
