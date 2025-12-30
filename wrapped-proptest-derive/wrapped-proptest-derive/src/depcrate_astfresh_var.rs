// Generated macro for fresh_var (function)
macro_rules! Depcrate_astfresh_var {
() => {
// Module: crate::ast
// Provides: {"fresh_var"}
// Dependencies: {}
# [doc = " Construct a `FreshVar` with the given `prefix` and the number it has in the"] # [doc = " count of temporaries for that prefix."] fn fresh_var (prefix : & str , count : usize) -> FreshVar < '_ > { FreshVar { prefix , count } }
};
}
