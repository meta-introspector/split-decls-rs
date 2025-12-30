// Generated macro for warn (macro)
macro_rules! Depcratewarn {
() => {
// Module: crate
// Provides: {"warn"}
// Dependencies: {}
# [doc = " Logs a message at the warn level."] # [macro_export (local_inner_macros)] macro_rules ! warn { (target : $ target : expr , $ ($ arg : tt) +) => (log ! (target : $ target , $ crate :: Level :: Warn , $ ($ arg) +) ;) ; ($ ($ arg : tt) +) => (log ! ($ crate :: Level :: Warn , $ ($ arg) +) ;) }
};
}
