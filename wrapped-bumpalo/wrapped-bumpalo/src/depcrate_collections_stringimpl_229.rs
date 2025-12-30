// Generated macro for impl_229 (impl)
macro_rules! Depcrate_collections_stringimpl_229 {
() => {
// Module: crate::collections::string
// Provides: {"impl_229"}
// Dependencies: {}
impl < 'bump > ops :: Deref for String < 'bump > { type Target = str ; # [inline] fn deref (& self) -> & str { unsafe { str :: from_utf8_unchecked (& self . vec) } } }
};
}
