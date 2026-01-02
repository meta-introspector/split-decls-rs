// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_llvm/src/lib.rs
// Error: expected square brackets
// Problematic line: line 14


use libc::size_t;

unsafe extern "C" {
    /// Opaque type that allows C++ code to write bytes to a Rust-side buffer,
    /// in conjunction with `RawRustStringOstream`. Use this as `&RustString`
    /// (Rust) and `RustStringRef` (C++) in FFI signatures.
