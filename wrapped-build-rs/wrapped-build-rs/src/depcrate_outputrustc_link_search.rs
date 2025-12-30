// Generated macro for rustc_link_search (function)
macro_rules! Depcrate_outputrustc_link_search {
() => {
// Module: crate::output
// Provides: {"rustc_link_search"}
// Dependencies: {}
# [doc = " The `rustc-link-search` instruction tells Cargo to pass the [`-L` flag] to the"] # [doc = " compiler to add a directory to the library search path."] # [doc = ""] # [doc = " The optional `KIND` may be one of `dependency`, `crate`, `native`, `framework`,"] # [doc = " or `all`. See the [rustc book][-L] for more detail."] # [doc = ""] # [doc = " These paths are also added to the"] # [doc = " [dynamic library search path environment variable][search-path] if they are"] # [doc = " within the `OUT_DIR`. Depending on this behavior is discouraged since this"] # [doc = " makes it difficult to use the resulting binary. In general, it is best to"] # [doc = " avoid creating dynamic libraries in a build script (using existing system"] # [doc = " libraries is fine)."] # [doc = ""] # [doc = " [-L]: https://doc.rust-lang.org/stable/rustc/command-line-arguments.html#option-l-search-path"] # [doc = " [search-path]: https://doc.rust-lang.org/stable/cargo/reference/environment-variables.html#dynamic-library-paths"] # [track_caller] pub fn rustc_link_search (path : impl AsRef < Path >) { let Some (path) = path . as_ref () . to_str () else { panic ! ("cannot emit rustc-link-search: path is not UTF-8") ; } ; if path . contains ('\n') { panic ! ("cannot emit rustc-link-search: path contains newline") ; } emit ("rustc-link-search" , path) ; }
};
}
