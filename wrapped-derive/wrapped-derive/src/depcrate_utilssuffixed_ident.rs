// Generated macro for suffixed_ident (function)
macro_rules! Depcrate_utilssuffixed_ident {
() => {
// Module: crate::utils
// Provides: {"suffixed_ident"}
// Dependencies: {}
fn suffixed_ident (name : & str , suffix : usize , s : Span) -> Ident { Ident :: new (& format ! ("{name}_{suffix}") , s) }
};
}
