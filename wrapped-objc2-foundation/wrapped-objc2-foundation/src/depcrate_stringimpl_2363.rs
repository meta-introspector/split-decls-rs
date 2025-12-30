// Generated macro for impl_2363 (impl)
macro_rules! Depcrate_stringimpl_2363 {
() => {
// Module: crate::string
// Provides: {"impl_2363"}
// Dependencies: {}
impl NSMutableString { # [doc = " Creates a new [`NSMutableString`] by copying the given string slice."] # [doc (alias = "initWithBytes:length:encoding:")] # [allow (clippy :: should_implement_trait)] pub fn from_str (string : & str) -> Retained < Self > { unsafe { init_with_str (Self :: alloc () , string) } } }
};
}
