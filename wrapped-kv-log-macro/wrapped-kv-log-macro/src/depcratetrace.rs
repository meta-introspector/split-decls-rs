// Generated macro for trace (macro)
macro_rules! Depcratetrace {
() => {
// Module: crate
// Provides: {"trace"}
// Dependencies: {}
# [doc = " Logs a message at the trace level."] # [macro_export (local_inner_macros)] macro_rules ! trace { (target : $ target : expr , $ ($ arg : tt) +) => (log ! (target : $ target , $ crate :: Level :: Trace , $ ($ arg) +) ;) ; ($ ($ arg : tt) +) => (log ! ($ crate :: Level :: Trace , $ ($ arg) +) ;) }
};
}
