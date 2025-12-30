// Generated macro for impl_30 (impl)
macro_rules! Depcrate_macrosimpl_30 {
() => {
// Module: crate::macros
// Provides: {"impl_30"}
// Dependencies: {}
impl MediaType { # [doc = " **DO NOT CALL THIS FUNCTION.**"] # [doc = ""] # [doc = " This function has no backwards-compatibility guarantees. It can and"] # [doc = " *will* change, and your code *will* break."] # [doc = " Kittens **will** die."] # [doc = ""] # [doc = " # Tests"] # [doc = ""] # [doc = " ```"] # [doc = " let foo = mime::media_type!(\"text/foo\");"] # [doc = " assert_eq!(foo.type_(), mime::TEXT);"] # [doc = " assert_eq!(foo.subtype(), \"foo\");"] # [doc = " assert_eq!(foo.suffix(), None);"] # [doc = " assert!(!foo.has_params());"] # [doc = " ```"] # [doc = ""] # [doc = " # Uppercase"] # [doc = ""] # [doc = " ```"] # [doc = " mime::media_type!(\"TEXT/PLAIN\");"] # [doc = " ```"] # [doc = ""] # [doc = " # Parameters"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " mime::media_type!(\"multipart/form-data; boundary=abcd; two=2\");"] # [doc = " ```"] # [doc = ""] # [doc = " # Ranges"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " mime::media_type!(\"text/*\");"] # [doc = " ```"] # [doc = ""] # [doc = " # String literal"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " mime::media_type!(text/foo);"] # [doc = " ```"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " mime::media_type!(\"text/foo\", \"+json\");"] # [doc = " ```"] # [doc = ""] # [doc = " # Dynamic Formatting"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " mime::media_type!(\"text/foo+{}\", \"json\");"] # [doc = " ```"] # [doc (hidden)] # [cfg (feature = "macro")] pub const unsafe fn private_from_proc_macro (mime : crate :: private :: Mime ,) -> Self { MediaType { mime , } } }
};
}
