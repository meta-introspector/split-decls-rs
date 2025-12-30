// Generated macro for impl_660 (impl)
macro_rules! Depcrate_ir_entitiesimpl_660 {
() => {
// Module: crate::ir::entities
// Provides: {"impl_660"}
// Dependencies: {}
impl MemoryType { # [doc = " Create a new memory type reference from its number."] # [doc = ""] # [doc = " This method is for use by the parser."] pub fn with_number (n : u32) -> Option < Self > { if n < u32 :: MAX { Some (Self (n)) } else { None } } }
};
}
