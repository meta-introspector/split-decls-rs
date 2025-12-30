// Generated macro for SourceType (enum)
macro_rules! Depcrate_builderSourceType {
() => {
// Module: crate::builder
// Provides: {"SourceType"}
// Dependencies: {}
# [derive (Debug , Clone)] enum SourceType { Sync (Box < dyn Source + Send + Sync >) , # [cfg (feature = "async")] Async (Box < dyn AsyncSource + Send + Sync >) , }
};
}
