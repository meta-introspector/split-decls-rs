// Generated macro for impl_648 (impl)
macro_rules! Depcrate_ir_entitiesimpl_648 {
() => {
// Module: crate::ir::entities
// Provides: {"impl_648"}
// Dependencies: {}
impl StackSlot { # [doc = " Create a new stack slot reference from its number."] # [doc = ""] # [doc = " This method is for use by the parser."] pub fn with_number (n : u32) -> Option < Self > { if n < u32 :: MAX { Some (Self (n)) } else { None } } }
};
}
