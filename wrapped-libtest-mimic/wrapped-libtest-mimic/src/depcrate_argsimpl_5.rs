// Generated macro for impl_5 (impl)
macro_rules! Depcrate_argsimpl_5 {
() => {
// Module: crate::args
// Provides: {"impl_5"}
// Dependencies: {}
impl Arguments { # [doc = " Parses the global CLI arguments given to the application."] # [doc = ""] # [doc = " If the parsing fails (due to incorrect CLI args), an error is shown and"] # [doc = " the application exits. If help is requested (`-h` or `--help`), a help"] # [doc = " message is shown and the application exits, too."] pub fn from_args () -> Self { Parser :: parse () } # [doc = " Like `from_args()`, but operates on an explicit iterator and not the"] # [doc = " global arguments. Note that the first element is the executable name!"] pub fn from_iter < I > (iter : I) -> Self where Self : Sized , I : IntoIterator , I :: Item : Into < std :: ffi :: OsString > + Clone , { Parser :: parse_from (iter) } }
};
}
