// Generated macro for impl_219 (impl)
macro_rules! Depcrate_stringsimpl_219 {
() => {
// Module: crate::strings
// Provides: {"impl_219"}
// Dependencies: {}
impl Deref for InlineStr { type Target = str ; fn deref (& self) -> & str { let len = self . len as usize ; from_utf8 (& self . inner [.. len]) . unwrap () } }
};
}
