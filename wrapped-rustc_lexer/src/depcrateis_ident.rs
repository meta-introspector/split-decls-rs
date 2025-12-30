// Generated macro for is_ident (function)
macro_rules! Depcrateis_ident {
() => {
// Module: crate
// Provides: {"is_ident"}
// Dependencies: {}
# [doc = " The passed string is lexically an identifier."] pub fn is_ident (string : & str) -> bool { let mut chars = string . chars () ; if let Some (start) = chars . next () { is_id_start (start) && chars . all (is_id_continue) } else { false } }
};
}
