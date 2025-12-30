// Generated macro for ready (macro)
macro_rules! Depcrateready {
() => {
// Module: crate
// Provides: {"ready"}
// Dependencies: {}
macro_rules ! ready { ($ e : expr) => { { match $ e { Poll :: Ready (t) => t , Poll :: Pending => return Poll :: Pending , } } } ; }
};
}
