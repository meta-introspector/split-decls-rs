// Generated macro for impl_1771 (impl)
macro_rules! Depcrate_stringimpl_1771 {
() => {
// Module: crate::string
// Provides: {"impl_1771"}
// Dependencies: {}
impl Ord for CFString { # [inline] # [doc (alias = "CFStringCompare")] fn cmp (& self , other : & Self) -> Ordering { let flags = CFStringCompareFlags :: empty () ; self . compare (Some (other) , flags) . into () } }
};
}
