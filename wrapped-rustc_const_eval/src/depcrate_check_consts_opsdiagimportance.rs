// Generated macro for DiagImportance (enum)
macro_rules! Depcrate_check_consts_opsDiagImportance {
() => {
// Module: crate::check_consts::ops
// Provides: {"DiagImportance"}
// Dependencies: {}
# [derive (Clone , Copy)] pub enum DiagImportance { # [doc = " An operation that must be removed for const-checking to pass."] Primary , # [doc = " An operation that causes const-checking to fail, but is usually a side-effect of a `Primary` operation elsewhere."] Secondary , }
};
}
