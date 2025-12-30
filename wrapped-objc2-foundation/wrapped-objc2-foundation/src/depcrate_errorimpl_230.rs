// Generated macro for impl_230 (impl)
macro_rules! Depcrate_errorimpl_230 {
() => {
// Module: crate::error
// Provides: {"impl_230"}
// Dependencies: {}
# [doc = " Creation methods."] impl NSError { # [doc = " Construct a new [`NSError`] with the given code in the given domain."] # [cfg (feature = "NSDictionary")] # [cfg (feature = "NSString")] pub fn new (code : objc2 :: ffi :: NSInteger , domain : & crate :: NSErrorDomain ,) -> objc2 :: rc :: Retained < Self > { use objc2 :: AnyThread ; unsafe { Self :: initWithDomain_code_userInfo (Self :: alloc () , domain , code , None) } } }
};
}
