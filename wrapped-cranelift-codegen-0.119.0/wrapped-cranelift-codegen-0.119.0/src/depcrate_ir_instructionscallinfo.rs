// Generated macro for CallInfo (enum)
macro_rules! Depcrate_ir_instructionsCallInfo {
() => {
// Module: crate::ir::instructions
// Provides: {"CallInfo"}
// Dependencies: {}
# [doc = " Information about call instructions."] pub enum CallInfo < 'a > { # [doc = " This is not a call instruction."] NotACall , # [doc = " This is a direct call to an external function declared in the preamble. See"] # [doc = " `DataFlowGraph.ext_funcs`."] Direct (FuncRef , & 'a [Value]) , # [doc = " This is an indirect call with the specified signature. See `DataFlowGraph.signatures`."] Indirect (SigRef , & 'a [Value]) , }
};
}
