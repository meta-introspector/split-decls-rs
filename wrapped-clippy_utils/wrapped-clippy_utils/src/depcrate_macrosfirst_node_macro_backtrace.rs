// Generated macro for first_node_macro_backtrace (function)
macro_rules! Depcrate_macrosfirst_node_macro_backtrace {
() => {
// Module: crate::macros
// Provides: {"first_node_macro_backtrace"}
// Dependencies: {}
# [doc = " Like [`macro_backtrace`], but only returns macro calls where `node` is the \"first node\" of the"] # [doc = " macro call, as in [`first_node_in_macro`]."] pub fn first_node_macro_backtrace (cx : & LateContext < '_ > , node : & impl HirNode) -> impl Iterator < Item = MacroCall > { let span = node . span () ; first_node_in_macro (cx , node) . into_iter () . flat_map (move | expn | macro_backtrace (span) . take_while (move | macro_call | macro_call . expn != expn)) }
};
}
