// Generated macro for simple_visit (function)
macro_rules! Depcrate_visit_mutsimple_visit {
() => {
// Module: crate::visit_mut
// Provides: {"simple_visit"}
// Dependencies: {}
fn simple_visit (item : & str , name : & Operand) -> TokenStream { let ident = gen :: under_name (item) ; let method = format_ident ! ("visit_{}_mut" , ident) ; let name = name . ref_mut_tokens () ; quote ! { v .# method (# name) } }
};
}
