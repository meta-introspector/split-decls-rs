// Generated macro for Context (struct)
macro_rules! Depcrate_contextContext {
() => {
// Module: crate::context
// Provides: {"Context"}
// Dependencies: {}
# [doc = " Persistent data structures and compilation pipeline."] pub struct Context { # [doc = " The function we're compiling."] pub func : Function , # [doc = " The control flow graph of `func`."] pub cfg : ControlFlowGraph , # [doc = " Dominator tree for `func`."] pub domtree : DominatorTree , # [doc = " Loop analysis of `func`."] pub loop_analysis : LoopAnalysis , # [doc = " Result of MachBackend compilation, if computed."] pub (crate) compiled_code : Option < CompiledCode > , # [doc = " Flag: do we want a disassembly with the CompiledCode?"] pub want_disasm : bool , }
};
}
