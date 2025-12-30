// Generated macro for impl_651 (impl)
macro_rules! Depcrate_ir_entitiesimpl_651 {
() => {
// Module: crate::ir::entities
// Provides: {"impl_651"}
// Dependencies: {}
impl DynamicStackSlot { # [doc = " Create a new stack slot reference from its number."] # [doc = ""] # [doc = " This method is for use by the parser."] pub fn with_number (n : u32) -> Option < Self > { if n < u32 :: MAX { Some (Self (n)) } else { None } } }
};
}
