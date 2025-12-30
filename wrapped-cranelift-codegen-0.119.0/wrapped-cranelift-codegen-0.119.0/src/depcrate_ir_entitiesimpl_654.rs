// Generated macro for impl_654 (impl)
macro_rules! Depcrate_ir_entitiesimpl_654 {
() => {
// Module: crate::ir::entities
// Provides: {"impl_654"}
// Dependencies: {}
impl DynamicType { # [doc = " Create a new dynamic type reference from its number."] # [doc = ""] # [doc = " This method is for use by the parser."] pub fn with_number (n : u32) -> Option < Self > { if n < u32 :: MAX { Some (Self (n)) } else { None } } }
};
}
