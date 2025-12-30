// Generated macro for deanonymize_lifetime (function)
macro_rules! Depcratedeanonymize_lifetime {
() => {
// Module: crate
// Provides: {"deanonymize_lifetime"}
// Dependencies: {}
fn deanonymize_lifetime (lt : & mut Lifetime) { if lt . ident == "_" { lt . ident = format_ident ! ("static") ; } }
};
}
