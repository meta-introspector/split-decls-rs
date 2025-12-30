// Generated macro for AliasAnalysis (struct)
macro_rules! Depcrate_alias_analysisAliasAnalysis {
() => {
// Module: crate::alias_analysis
// Provides: {"AliasAnalysis"}
// Dependencies: {}
# [doc = " An alias-analysis pass."] pub struct AliasAnalysis < 'a > { # [doc = " The domtree for the function."] domtree : & 'a DominatorTree , # [doc = " Input state to a basic block."] block_input : FxHashMap < Block , LastStores > , # [doc = " Known memory-value equivalences. This is the result of the"] # [doc = " analysis. This is a mapping from (last store, address"] # [doc = " expression, offset, type) to SSA `Value`."] # [doc = ""] # [doc = " We keep the defining inst around for quick dominance checks."] mem_values : FxHashMap < MemoryLoc , (Inst , Value) > , }
};
}
