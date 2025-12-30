// Generated macro for p (function)
macro_rules! Depcrate_exprp {
() => {
// Module: crate::expr
// Provides: {"p"}
// Dependencies: {}
fn p (c : & 'static str) -> impl Fn (& [Token]) -> CResult < '_ , & [u8] > { exact_token ! (Punctuation , c . as_bytes ()) }
};
}
