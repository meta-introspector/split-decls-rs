// Generated macro for impl_40 (impl)
macro_rules! Depcrate_attributed_stringimpl_40 {
() => {
// Module: crate::attributed_string
// Provides: {"impl_40"}
// Dependencies: {}
impl CFMutableAttributedString { # [inline] pub fn new () -> Self { unsafe { let astr_ref = CFAttributedStringCreateMutable (kCFAllocatorDefault , 0) ; CFMutableAttributedString :: wrap_under_create_rule (astr_ref) } } # [inline] pub fn char_len (& self) -> CFIndex { unsafe { CFAttributedStringGetLength (self . 0) } } # [inline] pub fn replace_str (& mut self , string : & CFString , range : CFRange) { unsafe { CFAttributedStringReplaceString (self . 0 , range , string . as_concrete_TypeRef ()) ; } } # [inline] pub fn set_attribute < T : TCFType > (& mut self , range : CFRange , name : CFStringRef , value : & T) { unsafe { CFAttributedStringSetAttribute (self . 0 , range , name , value . as_CFTypeRef ()) ; } } }
};
}
