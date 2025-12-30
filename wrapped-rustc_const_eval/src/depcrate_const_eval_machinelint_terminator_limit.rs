// Generated macro for LINT_TERMINATOR_LIMIT (const)
macro_rules! Depcrate_const_eval_machineLINT_TERMINATOR_LIMIT {
() => {
// Module: crate::const_eval::machine
// Provides: {"LINT_TERMINATOR_LIMIT"}
// Dependencies: {}
# [doc = " When hitting this many interpreted terminators we emit a deny by default lint"] # [doc = " that notfies the user that their constant takes a long time to evaluate. If that's"] # [doc = " what they intended, they can just allow the lint."] const LINT_TERMINATOR_LIMIT : usize = 2_000_000 ;
};
}
