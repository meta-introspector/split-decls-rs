// Generated macro for impl_187 (impl)
macro_rules! Depcrate_stringimpl_187 {
() => {
// Module: crate::string
// Provides: {"impl_187"}
// Dependencies: {}
impl Deref for CharStr { type Target = str ; fn deref (& self) -> & str { unsafe { str :: from_utf8_unchecked (& self . buf [.. self . len as usize]) } } }
};
}
