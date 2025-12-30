// Generated macro for call_with2 (function)
macro_rules! Depcrate_streamcall_with2 {
() => {
// Module: crate::stream
// Provides: {"call_with2"}
// Dependencies: {}
# [doc (hidden)] pub fn call_with2 < F , A , B , R > (a : A , b : B , f : F) -> R where F : FnOnce (A , B) -> R , { f (a , b) }
};
}
