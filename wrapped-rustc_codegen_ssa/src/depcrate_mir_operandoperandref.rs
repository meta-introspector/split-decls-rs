// Generated macro for OperandRef (struct)
macro_rules! Depcrate_mir_operandOperandRef {
() => {
// Module: crate::mir::operand
// Provides: {"OperandRef"}
// Dependencies: {}
# [doc = " An `OperandRef` is an \"SSA\" reference to a Rust value, along with"] # [doc = " its type."] # [doc = ""] # [doc = " NOTE: unless you know a value's type exactly, you should not"] # [doc = " generate LLVM opcodes acting on it and instead act via methods,"] # [doc = " to avoid nasty edge cases. In particular, using `Builder::store`"] # [doc = " directly is sure to cause problems -- use `OperandRef::store`"] # [doc = " instead."] # [derive (Copy , Clone)] pub struct OperandRef < 'tcx , V > { # [doc = " The value."] pub val : OperandValue < V > , # [doc = " The layout of value, based on its Rust type."] pub layout : TyAndLayout < 'tcx > , }
};
}
