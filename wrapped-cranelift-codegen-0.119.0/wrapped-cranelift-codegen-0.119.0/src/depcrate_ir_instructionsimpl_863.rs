// Generated macro for impl_863 (impl)
macro_rules! Depcrate_ir_instructionsimpl_863 {
() => {
// Module: crate::ir::instructions
// Provides: {"impl_863"}
// Dependencies: {}
impl < 'a > Table < & 'a str > for [Option < Opcode >] { fn len (& self) -> usize { self . len () } fn key (& self , idx : usize) -> Option < & 'a str > { self [idx] . map (opcode_name) } }
};
}
