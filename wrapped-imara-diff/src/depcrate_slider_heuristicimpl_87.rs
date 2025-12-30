// Generated macro for impl_87 (impl)
macro_rules! Depcrate_slider_heuristicimpl_87 {
() => {
// Module: crate::slider_heuristic
// Provides: {"impl_87"}
// Dependencies: {}
impl Score { fn for_range (range : Range < u32 > , tokens : & [Token] , indent_of_token : impl Fn (Token) -> IndentLevel ,) -> Score { Indents :: at_token (tokens , range . start as usize , & indent_of_token) . score () + Indents :: at_token (tokens , range . end as usize , & indent_of_token) . score () } }
};
}
