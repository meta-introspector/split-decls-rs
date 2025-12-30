// Generated macro for EarlyContext (struct)
macro_rules! Depcrate_contextEarlyContext {
() => {
// Module: crate::context
// Provides: {"EarlyContext"}
// Dependencies: {}
# [doc = " Context for lint checking of the AST, after expansion, before lowering to HIR."] pub struct EarlyContext < 'a > { pub builder : LintLevelsBuilder < 'a , crate :: levels :: TopDown > , pub buffered : LintBuffer , }
};
}
