// Generated macro for validate_ident_raw (function)
macro_rules! Depcrate_fallbackvalidate_ident_raw {
() => {
// Module: crate::fallback
// Provides: {"validate_ident_raw"}
// Dependencies: {}
# [track_caller] fn validate_ident_raw (string : & str) { validate_ident (string) ; match string { "_" | "super" | "self" | "Self" | "crate" => { panic ! ("`r#{}` cannot be a raw identifier" , string) ; } _ => { } } }
};
}
