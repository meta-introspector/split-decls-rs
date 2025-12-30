// Generated macro for simple_visit (function)
macro_rules! Depcrate_visitsimple_visit {
() => {
// Module: crate::visit
// Provides: {"simple_visit"}
// Dependencies: {}
fn simple_visit (item : & str , name : & Operand) -> TokenStream { let ident = gen :: under_name (item) ; let method = format_ident ! ("visit_{}" , ident) ; let name = name . ref_tokens () ; quote ! { v .# method (# name) } }
};
}
