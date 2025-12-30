// Generated macro for impl_1309 (impl)
macro_rules! Depcrate_stringimpl_1309 {
() => {
// Module: crate::string
// Provides: {"impl_1309"}
// Dependencies: {}
impl Deref for OpensslStringRef { type Target = str ; fn deref (& self) -> & str { unsafe { let slice = CStr :: from_ptr (self . as_ptr ()) . to_bytes () ; str :: from_utf8_unchecked (slice) } } }
};
}
