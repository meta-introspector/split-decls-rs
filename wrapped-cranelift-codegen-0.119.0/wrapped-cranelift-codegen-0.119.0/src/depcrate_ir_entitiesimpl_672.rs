// Generated macro for impl_672 (impl)
macro_rules! Depcrate_ir_entitiesimpl_672 {
() => {
// Module: crate::ir::entities
// Provides: {"impl_672"}
// Dependencies: {}
impl FuncRef { # [doc = " Create a new external function reference from its number."] # [doc = ""] # [doc = " This method is for use by the parser."] pub fn with_number (n : u32) -> Option < Self > { if n < u32 :: MAX { Some (Self (n)) } else { None } } }
};
}
