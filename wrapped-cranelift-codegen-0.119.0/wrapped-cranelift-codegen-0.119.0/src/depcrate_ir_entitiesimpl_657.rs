// Generated macro for impl_657 (impl)
macro_rules! Depcrate_ir_entitiesimpl_657 {
() => {
// Module: crate::ir::entities
// Provides: {"impl_657"}
// Dependencies: {}
impl GlobalValue { # [doc = " Create a new global value reference from its number."] # [doc = ""] # [doc = " This method is for use by the parser."] pub fn with_number (n : u32) -> Option < Self > { if n < u32 :: MAX { Some (Self (n)) } else { None } } }
};
}
