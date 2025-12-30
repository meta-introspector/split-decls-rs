// Generated macro for Clang (struct)
macro_rules! Depcrate_supportClang {
() => {
// Module: crate::support
// Provides: {"Clang"}
// Dependencies: {}
# [doc = " A `clang` executable."] # [derive (Clone , Debug)] pub struct Clang { # [doc = " The path to this `clang` executable."] pub path : PathBuf , # [doc = " The version of this `clang` executable if it could be parsed."] pub version : Option < CXVersion > , # [doc = " The directories searched by this `clang` executable for C headers if"] # [doc = " they could be parsed."] pub c_search_paths : Option < Vec < PathBuf > > , # [doc = " The directories searched by this `clang` executable for C++ headers if"] # [doc = " they could be parsed."] pub cpp_search_paths : Option < Vec < PathBuf > > , }
};
}
