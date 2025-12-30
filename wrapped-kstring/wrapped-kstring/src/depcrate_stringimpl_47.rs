// Generated macro for impl_47 (impl)
macro_rules! Depcrate_stringimpl_47 {
() => {
// Module: crate::string
// Provides: {"impl_47"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > Ord for KStringBase < B > { # [inline] fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . as_str () . cmp (other . as_str ()) } }
};
}
