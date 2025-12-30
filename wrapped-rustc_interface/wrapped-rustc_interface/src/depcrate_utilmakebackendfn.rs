// Generated macro for MakeBackendFn (type)
macro_rules! Depcrate_utilMakeBackendFn {
() => {
// Module: crate::util
// Provides: {"MakeBackendFn"}
// Dependencies: {}
# [doc = " Function pointer type that constructs a new CodegenBackend."] type MakeBackendFn = fn () -> Box < dyn CodegenBackend > ;
};
}
