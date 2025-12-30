// Generated macro for really_warn (macro)
macro_rules! Depcrate_countersreally_warn {
() => {
// Module: crate::counters
// Provides: {"really_warn"}
// Dependencies: {}
macro_rules ! really_warn { ($ msg : literal $ ($ rest : tt) *) => { error ! (concat ! ("[WARNING] " , $ msg) $ ($ rest) *) } }
};
}
