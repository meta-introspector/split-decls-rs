// Generated macro for __fallback_ensure (macro)
macro_rules! Depcrate_ensure__fallback_ensure {
() => {
// Module: crate::ensure
// Provides: {"__fallback_ensure"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __fallback_ensure { ($ cond : expr $ (,) ?) => { if $ crate :: __private :: not ($ cond) { return $ crate :: __private :: Err ($ crate :: Error :: msg ($ crate :: __private :: concat ! ("Condition failed: `" , $ crate :: __private :: stringify ! ($ cond) , "`"))) ; } } ; ($ cond : expr , $ msg : literal $ (,) ?) => { if $ crate :: __private :: not ($ cond) { return $ crate :: __private :: Err ($ crate :: __anyhow ! ($ msg)) ; } } ; ($ cond : expr , $ err : expr $ (,) ?) => { if $ crate :: __private :: not ($ cond) { return $ crate :: __private :: Err ($ crate :: __anyhow ! ($ err)) ; } } ; ($ cond : expr , $ fmt : expr , $ ($ arg : tt) *) => { if $ crate :: __private :: not ($ cond) { return $ crate :: __private :: Err ($ crate :: __anyhow ! ($ fmt , $ ($ arg) *)) ; } } ; }
};
}
