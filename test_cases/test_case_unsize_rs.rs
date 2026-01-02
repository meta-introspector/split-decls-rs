// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/unsize.rs
// Error: expected square brackets
// Problematic line: line 14


// Adapted from https://github.com/rust-lang/rust/blob/2a663555ddf36f6b041445894a8c175cd1bc718c/src/librustc_codegen_ssa/base.rs#L159-L307

/// Retrieve the information we are losing (making dynamic) in an unsizing
/// adjustment.
///
/// The `old_info` argument is a bit funny. It is intended for use
