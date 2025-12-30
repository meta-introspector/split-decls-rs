// Generated macro for BindCollector (type)
macro_rules! Depcrate_backendBindCollector {
() => {
// Module: crate::backend
// Provides: {"BindCollector"}
// Dependencies: {}
# [doc (hidden)] # [cfg (all (feature = "with-deprecated" , not (feature = "without-deprecated")))] # [deprecated (note = "Use `Backend::BindCollector` directly")] pub type BindCollector < 'a , DB > = < DB as Backend > :: BindCollector < 'a > ;
};
}
