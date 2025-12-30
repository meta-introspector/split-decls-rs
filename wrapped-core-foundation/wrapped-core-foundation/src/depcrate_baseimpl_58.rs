// Generated macro for impl_58 (impl)
macro_rules! Depcrate_baseimpl_58 {
() => {
// Module: crate::base
// Provides: {"impl_58"}
// Dependencies: {}
impl fmt :: Debug for CFType { # [doc = " Formats the value using [`CFCopyDescription`]."] # [doc = ""] # [doc = " [`CFCopyDescription`]: https://developer.apple.com/documentation/corefoundation/1521252-cfcopydescription?language=objc"] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let desc = unsafe { CFString :: wrap_under_create_rule (CFCopyDescription (self . 0)) } ; desc . fmt (f) } }
};
}
