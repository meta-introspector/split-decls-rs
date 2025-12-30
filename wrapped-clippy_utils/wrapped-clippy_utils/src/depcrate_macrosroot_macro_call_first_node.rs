// Generated macro for root_macro_call_first_node (function)
macro_rules! Depcrate_macrosroot_macro_call_first_node {
() => {
// Module: crate::macros
// Provides: {"root_macro_call_first_node"}
// Dependencies: {}
# [doc = " Like [`root_macro_call`], but only returns `Some` if `node` is the \"first node\""] # [doc = " produced by the macro call, as in [`first_node_in_macro`]."] pub fn root_macro_call_first_node (cx : & LateContext < '_ > , node : & impl HirNode) -> Option < MacroCall > { if first_node_in_macro (cx , node) != Some (ExpnId :: root ()) { return None ; } root_macro_call (node . span ()) }
};
}
