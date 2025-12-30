// Generated macro for has_cfg_test (function)
macro_rules! Depcrate_runnableshas_cfg_test {
() => {
// Module: crate::runnables
// Provides: {"has_cfg_test"}
// Dependencies: {}
fn has_cfg_test (attrs : AttrsWithOwner) -> bool { attrs . cfgs () . any (| cfg | matches ! (& cfg , CfgExpr :: Atom (CfgAtom :: Flag (s)) if * s == sym :: test)) }
};
}
