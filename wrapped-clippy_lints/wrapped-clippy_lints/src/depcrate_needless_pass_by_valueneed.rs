// Generated macro for need (macro)
macro_rules! Depcrate_needless_pass_by_valueneed {
() => {
// Module: crate::needless_pass_by_value
// Provides: {"need"}
// Dependencies: {}
macro_rules ! need { ($ e : expr) => { if let Some (x) = $ e { x } else { return ; } } ; }
};
}
