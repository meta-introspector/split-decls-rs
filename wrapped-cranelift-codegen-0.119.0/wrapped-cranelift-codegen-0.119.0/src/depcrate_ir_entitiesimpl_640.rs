// Generated macro for impl_640 (impl)
macro_rules! Depcrate_ir_entitiesimpl_640 {
() => {
// Module: crate::ir::entities
// Provides: {"impl_640"}
// Dependencies: {}
impl Block { # [doc = " Create a new block reference from its number. This corresponds to the `blockNN` representation."] # [doc = ""] # [doc = " This method is for use by the parser."] pub fn with_number (n : u32) -> Option < Self > { if n < u32 :: MAX { Some (Self (n)) } else { None } } }
};
}
