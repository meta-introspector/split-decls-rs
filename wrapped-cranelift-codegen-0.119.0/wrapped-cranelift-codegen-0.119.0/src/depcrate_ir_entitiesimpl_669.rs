// Generated macro for impl_669 (impl)
macro_rules! Depcrate_ir_entitiesimpl_669 {
() => {
// Module: crate::ir::entities
// Provides: {"impl_669"}
// Dependencies: {}
impl JumpTable { # [doc = " Create a new jump table reference from its number."] # [doc = ""] # [doc = " This method is for use by the parser."] pub fn with_number (n : u32) -> Option < Self > { if n < u32 :: MAX { Some (Self (n)) } else { None } } }
};
}
