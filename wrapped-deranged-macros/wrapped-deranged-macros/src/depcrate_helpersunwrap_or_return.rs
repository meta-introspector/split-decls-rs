// Generated macro for unwrap_or_return (macro)
macro_rules! Depcrate_helpersunwrap_or_return {
() => {
// Module: crate::helpers
// Provides: {"unwrap_or_return"}
// Dependencies: {}
# [doc = " Unwrap a `Result` or return the error directly."] macro_rules ! unwrap_or_return { ($ e : expr) => { match $ e { Ok (value) => value , Err (err) => return err , } } ; }
};
}
