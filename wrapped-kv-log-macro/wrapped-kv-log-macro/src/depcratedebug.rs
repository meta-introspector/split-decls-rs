// Generated macro for debug (macro)
macro_rules! Depcratedebug {
() => {
// Module: crate
// Provides: {"debug"}
// Dependencies: {}
# [doc = " Logs a message at the debug level."] # [macro_export (local_inner_macros)] macro_rules ! debug { (target : $ target : expr , $ ($ arg : tt) +) => (log ! (target : $ target , $ crate :: Level :: Debug , $ ($ arg) +) ;) ; ($ ($ arg : tt) +) => (log ! ($ crate :: Level :: Debug , $ ($ arg) +) ;) }
};
}
