// Generated macro for OptimizedRule (struct)
macro_rules! Depcrate_optimizerOptimizedRule {
() => {
// Module: crate::optimizer
// Provides: {"OptimizedRule"}
// Dependencies: {}
# [doc = " The optimized version of the pest AST's `Rule`."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct OptimizedRule { # [doc = " The name of the rule."] pub name : String , # [doc = " The type of the rule."] pub ty : RuleType , # [doc = " The optimized expression of the rule."] pub expr : OptimizedExpr , }
};
}
