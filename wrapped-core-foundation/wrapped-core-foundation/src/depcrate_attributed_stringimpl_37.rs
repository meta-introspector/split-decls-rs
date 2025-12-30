// Generated macro for impl_37 (impl)
macro_rules! Depcrate_attributed_stringimpl_37 {
() => {
// Module: crate::attributed_string
// Provides: {"impl_37"}
// Dependencies: {}
impl CFAttributedString { # [inline] pub fn new (string : & CFString) -> Self { unsafe { let astr_ref = CFAttributedStringCreate (kCFAllocatorDefault , string . as_concrete_TypeRef () , null ()) ; CFAttributedString :: wrap_under_create_rule (astr_ref) } } # [inline] pub fn char_len (& self) -> CFIndex { unsafe { CFAttributedStringGetLength (self . 0) } } }
};
}
