// Generated macro for noop_visit (function)
macro_rules! Depcrate_visit_mutnoop_visit {
() => {
// Module: crate::visit_mut
// Provides: {"noop_visit"}
// Dependencies: {}
fn noop_visit (name : & Operand) -> TokenStream { let name = name . tokens () ; quote ! { skip ! (# name) } }
};
}
