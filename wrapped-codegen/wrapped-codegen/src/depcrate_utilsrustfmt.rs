// Generated macro for rustfmt (function)
macro_rules! Depcrate_utilsrustfmt {
() => {
// Module: crate::utils
// Provides: {"rustfmt"}
// Dependencies: {}
pub fn rustfmt (s : & str) -> String { match rustfmt_raw (s) { Ok (s) => s , Err (_) => s . replace ("}" , "}\n") , } }
};
}
