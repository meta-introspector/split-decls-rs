// Generated macro for impl_230 (impl)
macro_rules! Depcrate_collections_stringimpl_230 {
() => {
// Module: crate::collections::string
// Provides: {"impl_230"}
// Dependencies: {}
impl < 'bump > ops :: DerefMut for String < 'bump > { # [inline] fn deref_mut (& mut self) -> & mut str { unsafe { str :: from_utf8_unchecked_mut (& mut * self . vec) } } }
};
}
