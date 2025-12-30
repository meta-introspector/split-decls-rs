// Generated macro for impl_112 (impl)
macro_rules! Depcrate_string_cowimpl_112 {
() => {
// Module: crate::string_cow
// Provides: {"impl_112"}
// Dependencies: {}
impl < B : crate :: backend :: HeapStr > From < BoxedStr > for KStringCowBase < '_ , B > { # [inline] fn from (other : BoxedStr) -> Self { Self :: from_boxed (other) } }
};
}
