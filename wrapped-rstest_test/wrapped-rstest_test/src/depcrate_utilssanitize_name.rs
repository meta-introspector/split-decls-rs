// Generated macro for sanitize_name (function)
macro_rules! Depcrate_utilssanitize_name {
() => {
// Module: crate::utils
// Provides: {"sanitize_name"}
// Dependencies: {}
pub fn sanitize_name < S : AsRef < str > > (s : S) -> String { s . as_ref () . replace (':' , "_") . replace ("__" , "_") }
};
}
