// Generated macro for RemapScheme (enum)
macro_rules! DepcrateRemapScheme {
() => {
// Module: crate
// Provides: {"RemapScheme"}
// Dependencies: {}
# [doc = " When `rust.rust_remap_debuginfo` is requested, the compiler needs to know how to"] # [doc = " opportunistically unremap compiler vs non-compiler sources. We use two schemes,"] # [doc = " [`RemapScheme::Compiler`] and [`RemapScheme::NonCompiler`]."] pub enum RemapScheme { # [doc = " The [`RemapScheme::Compiler`] scheme will remap to `/rustc-dev/{hash}`."] Compiler , # [doc = " The [`RemapScheme::NonCompiler`] scheme will remap to `/rustc/{hash}`."] NonCompiler , }
};
}
