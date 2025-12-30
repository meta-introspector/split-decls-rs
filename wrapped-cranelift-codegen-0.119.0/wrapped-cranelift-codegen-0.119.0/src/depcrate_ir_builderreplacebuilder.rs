// Generated macro for ReplaceBuilder (struct)
macro_rules! Depcrate_ir_builderReplaceBuilder {
() => {
// Module: crate::ir::builder
// Provides: {"ReplaceBuilder"}
// Dependencies: {}
# [doc = " Instruction builder that replaces an existing instruction."] # [doc = ""] # [doc = " The inserted instruction will have the same `Inst` number as the old one."] # [doc = ""] # [doc = " If the old instruction still has result values attached, it is assumed that the new instruction"] # [doc = " produces the same number and types of results. The old result values are preserved. If the"] # [doc = " replacement instruction format does not support multiple results, the builder panics. It is a"] # [doc = " bug to leave result values dangling."] pub struct ReplaceBuilder < 'f > { dfg : & 'f mut DataFlowGraph , inst : Inst , }
};
}
