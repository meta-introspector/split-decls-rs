// Generated macro for impl_48 (impl)
macro_rules! Depcrate_stringimpl_48 {
() => {
// Module: crate::string
// Provides: {"impl_48"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > PartialOrd for KStringBase < B > { # [inline] fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } }
};
}
