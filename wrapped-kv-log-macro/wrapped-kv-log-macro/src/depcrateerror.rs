// Generated macro for error (macro)
macro_rules! Depcrateerror {
() => {
// Module: crate
// Provides: {"error"}
// Dependencies: {}
# [doc = " Logs a message at the error level."] # [macro_export (local_inner_macros)] macro_rules ! error { (target : $ target : expr , $ ($ arg : tt) +) => (log ! (target : $ target , $ crate :: Level :: Error , $ ($ arg) +) ;) ; ($ ($ arg : tt) +) => (log ! ($ crate :: Level :: Error , $ ($ arg) +) ;) }
};
}
