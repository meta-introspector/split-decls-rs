// Generated macro for impl_1194 (impl)
macro_rules! Depcrate_runtime_nsobjectimpl_1194 {
() => {
// Module: crate::runtime::nsobject
// Provides: {"impl_1194"}
// Dependencies: {}
# [doc = " Objective-C equality has approximately the same semantics as Rust"] # [doc = " equality (although less aptly specified)."] # [doc = ""] # [doc = " At the very least, equality is _expected_ to be symmetric and"] # [doc = " transitive, and that's about the best we can do."] # [doc = ""] # [doc = " See also <https://nshipster.com/equality/>"] impl PartialEq for NSObject { # [inline] # [doc (alias = "isEqual:")] fn eq (& self , other : & Self) -> bool { self . isEqual (Some (other)) } }
};
}
