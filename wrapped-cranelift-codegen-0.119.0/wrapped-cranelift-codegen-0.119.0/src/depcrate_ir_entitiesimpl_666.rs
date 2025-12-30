// Generated macro for impl_666 (impl)
macro_rules! Depcrate_ir_entitiesimpl_666 {
() => {
// Module: crate::ir::entities
// Provides: {"impl_666"}
// Dependencies: {}
impl Immediate { # [doc = " Create an immediate reference from its number."] # [doc = ""] # [doc = " This method is for use by the parser."] pub fn with_number (n : u32) -> Option < Self > { if n < u32 :: MAX { Some (Self (n)) } else { None } } }
};
}
