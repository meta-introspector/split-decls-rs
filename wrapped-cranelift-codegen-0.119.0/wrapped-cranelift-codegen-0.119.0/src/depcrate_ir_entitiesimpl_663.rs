// Generated macro for impl_663 (impl)
macro_rules! Depcrate_ir_entitiesimpl_663 {
() => {
// Module: crate::ir::entities
// Provides: {"impl_663"}
// Dependencies: {}
impl Constant { # [doc = " Create a const reference from its number."] # [doc = ""] # [doc = " This method is for use by the parser."] pub fn with_number (n : u32) -> Option < Self > { if n < u32 :: MAX { Some (Self (n)) } else { None } } }
};
}
