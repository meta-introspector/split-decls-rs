// Generated macro for info (macro)
macro_rules! Depcrateinfo {
() => {
// Module: crate
// Provides: {"info"}
// Dependencies: {}
# [doc = " Logs a message at the info level."] # [macro_export (local_inner_macros)] macro_rules ! info { (target : $ target : expr , $ ($ arg : tt) +) => (log ! (target : $ target , $ crate :: Level :: Info , $ ($ arg) +) ;) ; ($ ($ arg : tt) +) => (log ! ($ crate :: Level :: Info , $ ($ arg) +) ;) }
};
}
