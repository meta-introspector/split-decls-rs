// Generated macro for mut_pat (function)
macro_rules! Depcrate_receivermut_pat {
() => {
// Module: crate::receiver
// Provides: {"mut_pat"}
// Dependencies: {}
pub fn mut_pat (pat : & mut Pat) -> Option < Token ! [mut] > { let mut visitor = HasMutPat (None) ; visitor . visit_pat_mut (pat) ; visitor . 0 }
};
}
