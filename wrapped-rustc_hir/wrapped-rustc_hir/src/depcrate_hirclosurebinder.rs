// Generated macro for ClosureBinder (enum)
macro_rules! Depcrate_hirClosureBinder {
() => {
// Module: crate::hir
// Provides: {"ClosureBinder"}
// Dependencies: {}
# [doc = " Represents `for<...>` binder before a closure"] # [derive (Copy , Clone , Debug , HashStable_Generic)] pub enum ClosureBinder { # [doc = " Binder is not specified."] Default , # [doc = " Binder is specified."] # [doc = ""] # [doc = " Span points to the whole `for<...>`."] For { span : Span } , }
};
}
