// Generated macro for impl_86 (impl)
macro_rules! Depcrate_section_githubimpl_86 {
() => {
// Module: crate::section::github
// Provides: {"impl_86"}
// Dependencies: {}
impl Body { fn new () -> Self { Body { body : String :: new () , } } fn push_section < T > (& mut self , header : & 'static str , section : T) -> fmt :: Result where T : fmt :: Display , { use std :: fmt :: Write ; let separator = if self . body . is_empty () { "" } else { "\n\n" } ; let header = header . with_header ("## ") . with_header (separator) . with_footer ("\n") ; write ! (& mut self . body , "{}" , section . with_header (header)) } }
};
}
