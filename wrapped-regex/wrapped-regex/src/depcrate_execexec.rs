// Generated macro for Exec (struct)
macro_rules! Depcrate_execExec {
() => {
// Module: crate::exec
// Provides: {"Exec"}
// Dependencies: {}
# [doc = " Exec manages the execution of a regular expression."] # [doc = ""] # [doc = " In particular, this manages the various compiled forms of a single regular"] # [doc = " expression and the choice of which matching engine to use to execute a"] # [doc = " regular expression."] pub struct Exec { # [doc = " All read only state."] ro : Arc < ExecReadOnly > , # [doc = " Caches for the various matching engines."] cache : CachedThreadLocal < ProgramCache > , }
};
}
