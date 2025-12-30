// Generated macro for try_control (macro)
macro_rules! Depcrate_controltry_control {
() => {
// Module: crate::control
// Provides: {"try_control"}
// Dependencies: {}
macro_rules ! try_control { ($ e : expr) => { match $ e { x => { if x . should_break () { return x ; } } } } ; }
};
}
