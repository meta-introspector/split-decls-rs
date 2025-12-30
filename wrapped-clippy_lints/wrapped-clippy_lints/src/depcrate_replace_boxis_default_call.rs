// Generated macro for is_default_call (function)
macro_rules! Depcrate_replace_boxis_default_call {
() => {
// Module: crate::replace_box
// Provides: {"is_default_call"}
// Dependencies: {}
fn is_default_call (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { matches ! (expr . kind , ExprKind :: Call (func , _args) if is_default_equivalent_call (cx , func , Some (expr))) }
};
}
