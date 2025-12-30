// Generated macro for impl_18 (impl)
macro_rules! Depcrate_nameimpl_18 {
() => {
// Module: crate::name
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'a > Name { # [doc = " Provide our ref-type."] pub fn as_ref (& 'a self) -> NameRef < 'a > { NameRef (self . 0 . as_ref ()) } # [doc = " Return the inner `str`."] pub fn as_str (& self) -> & str { self . 0 . as_str () } }
};
}
