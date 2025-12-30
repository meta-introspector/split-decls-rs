// Generated macro for run (function)
macro_rules! Depcrate_fmtrun {
() => {
// Module: crate::fmt
// Provides: {"run"}
// Dependencies: {}
pub fn run (update_mode : UpdateMode) { run_rustfmt (update_mode) ; fmt_syms (update_mode) ; if let Err (e) = fmt_conf (update_mode . is_check ()) { e . display () ; process :: exit (1) ; } }
};
}
