// Generated macro for cmd (function)
macro_rules! Depcrate_execcmd {
() => {
// Module: crate::exec
// Provides: {"cmd"}
// Dependencies: {}
pub fn cmd (args : & [& str]) -> CmdBuilder { assert ! (! args . is_empty ()) ; CmdBuilder { args : args . iter () . map (| s | s . to_string ()) . collect () , .. Default :: default () } }
};
}
