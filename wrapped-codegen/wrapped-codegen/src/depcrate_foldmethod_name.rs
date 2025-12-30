// Generated macro for method_name (function)
macro_rules! Depcrate_foldmethod_name {
() => {
// Module: crate::fold
// Provides: {"method_name"}
// Dependencies: {}
fn method_name (item : & str) -> Ident { let ident = gen :: under_name (item) ; format_ident ! ("fold_{}" , ident) }
};
}
