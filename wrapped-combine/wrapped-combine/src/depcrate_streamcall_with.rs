// Generated macro for call_with (function)
macro_rules! Depcrate_streamcall_with {
() => {
// Module: crate::stream
// Provides: {"call_with"}
// Dependencies: {}
# [doc (hidden)] pub fn call_with < F , A , R > (a : A , f : F) -> R where F : FnOnce (A) -> R , { f (a) }
};
}
