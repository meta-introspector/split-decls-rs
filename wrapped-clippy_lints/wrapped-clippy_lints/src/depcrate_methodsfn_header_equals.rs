// Generated macro for fn_header_equals (function)
macro_rules! Depcrate_methodsfn_header_equals {
() => {
// Module: crate::methods
// Provides: {"fn_header_equals"}
// Dependencies: {}
fn fn_header_equals (expected : hir :: FnHeader , actual : hir :: FnHeader) -> bool { expected . constness == actual . constness && expected . safety == actual . safety && expected . asyncness == actual . asyncness }
};
}
