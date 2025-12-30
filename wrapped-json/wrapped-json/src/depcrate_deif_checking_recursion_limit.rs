// Generated macro for if_checking_recursion_limit (macro)
macro_rules! Depcrate_deif_checking_recursion_limit {
() => {
// Module: crate::de
// Provides: {"if_checking_recursion_limit"}
// Dependencies: {}
# [cfg (feature = "unbounded_depth")] macro_rules ! if_checking_recursion_limit { ($ this : ident $ ($ body : tt) *) => { if !$ this . disable_recursion_limit { $ this $ ($ body) * } } ; }
};
}
