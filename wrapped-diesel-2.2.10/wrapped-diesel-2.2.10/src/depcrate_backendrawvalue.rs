// Generated macro for RawValue (type)
macro_rules! Depcrate_backendRawValue {
() => {
// Module: crate::backend
// Provides: {"RawValue"}
// Dependencies: {}
# [doc (hidden)] # [cfg (all (feature = "with-deprecated" , not (feature = "without-deprecated")))] # [deprecated (note = "Use `Backend::RawValue` directly")] pub type RawValue < 'a , DB > = < DB as Backend > :: RawValue < 'a > ;
};
}
