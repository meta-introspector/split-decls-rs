// Generated macro for try_poll (macro)
macro_rules! Depcrate_polltry_poll {
() => {
// Module: crate::poll
// Provides: {"try_poll"}
// Dependencies: {}
# [doc = " A macro."] # [macro_export] macro_rules ! try_poll { ($ e : expr) => (match $ e { $ crate :: Poll :: NotReady => return $ crate :: Poll :: NotReady , $ crate :: Poll :: Ok (t) => Ok (t) , $ crate :: Poll :: Err (e) => Err (e) , }) }
};
}
