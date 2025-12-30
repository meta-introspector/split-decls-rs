// Generated macro for impl_677 (impl)
macro_rules! Depcrate_ir_entitiesimpl_677 {
() => {
// Module: crate::ir::entities
// Provides: {"impl_677"}
// Dependencies: {}
impl SigRef { # [doc = " Create a new function signature reference from its number."] # [doc = ""] # [doc = " This method is for use by the parser."] pub fn with_number (n : u32) -> Option < Self > { if n < u32 :: MAX { Some (Self (n)) } else { None } } }
};
}
