// Generated macro for rustc_link_lib (function)
macro_rules! Depcrate_outputrustc_link_lib {
() => {
// Module: crate::output
// Provides: {"rustc_link_lib"}
// Dependencies: {}
# [doc = " The `rustc-link-lib` instruction tells Cargo to link the given library using"] # [doc = " the compiler’s [`-l` flag][-l]."] # [doc = ""] # [doc = " This is typically used to link a native library"] # [doc = " using [FFI]."] # [doc = ""] # [doc = " The `LIB` string is passed directly to rustc, so it supports any syntax that"] # [doc = " `-l` does. Currently the full supported syntax for `LIB` is"] # [doc = " `[KIND[:MODIFIERS]=]NAME[:RENAME]`."] # [doc = ""] # [doc = " The `-l` flag is only passed to the library target of the package, unless there"] # [doc = " is no library target, in which case it is passed to all targets. This is done"] # [doc = " because all other targets have an implicit dependency on the library target,"] # [doc = " and the given library to link should only be included once. This means that"] # [doc = " if a package has both a library and a binary target, the library has access"] # [doc = " to the symbols from the given lib, and the binary should access them through"] # [doc = " the library target’s public API."] # [doc = ""] # [doc = " The optional `KIND` may be one of `dylib`, `static`, or `framework`. See the"] # [doc = " [rustc book][-l] for more detail."] # [doc = ""] # [doc = " [-l]: https://doc.rust-lang.org/stable/rustc/command-line-arguments.html#option-l-link-lib"] # [doc = " [FFI]: https://doc.rust-lang.org/stable/nomicon/ffi.html"] # [track_caller] pub fn rustc_link_lib (lib : & str) { if lib . contains ([' ' , '\n']) { panic ! ("cannot emit rustc-link-lib: invalid lib {lib:?}") ; } emit ("rustc-link-lib" , lib) ; }
};
}
