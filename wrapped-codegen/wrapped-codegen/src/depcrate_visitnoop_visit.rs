// Generated macro for noop_visit (function)
macro_rules! Depcrate_visitnoop_visit {
() => {
// Module: crate::visit
// Provides: {"noop_visit"}
// Dependencies: {}
fn noop_visit (name : & Operand) -> TokenStream { let name = name . tokens () ; quote ! { skip ! (# name) } }
};
}
