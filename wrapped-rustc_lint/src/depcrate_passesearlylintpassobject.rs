// Generated macro for EarlyLintPassObject (type)
macro_rules! Depcrate_passesEarlyLintPassObject {
() => {
// Module: crate::passes
// Provides: {"EarlyLintPassObject"}
// Dependencies: {}
# [doc = " A lint pass boxed up as a trait object."] pub (crate) type EarlyLintPassObject = Box < dyn EarlyLintPass + 'static > ;
};
}
