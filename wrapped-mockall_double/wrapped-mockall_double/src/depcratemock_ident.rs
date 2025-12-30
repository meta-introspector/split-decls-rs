// Generated macro for mock_ident (function)
macro_rules! Depcratemock_ident {
() => {
// Module: crate
// Provides: {"mock_ident"}
// Dependencies: {}
fn mock_ident (i : & Ident) -> Ident { let is_type = format ! ("{i}") . chars () . next () . expect ("zero-length ident?") . is_uppercase () ; if is_type { format_ident ! ("Mock{}" , i) } else { format_ident ! ("mock_{}" , i) } }
};
}
