// Generated macro for FactContext (struct)
macro_rules! Depcrate_ir_pccFactContext {
() => {
// Module: crate::ir::pcc
// Provides: {"FactContext"}
// Dependencies: {}
# [doc = " A \"context\" in which we can evaluate and derive facts. This"] # [doc = " context carries environment/global properties, such as the machine"] # [doc = " pointer width."] pub struct FactContext < 'a > { function : & 'a ir :: Function , pointer_width : u16 , }
};
}
