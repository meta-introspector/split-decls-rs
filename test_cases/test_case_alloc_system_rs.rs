// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_gcc/example/alloc_system.rs
// Error: expected square brackets
// Problematic line: line 9


// The minimum alignment guaranteed by the architecture. This value is used to
// add fast paths for low alignment values.
#[cfg(any(target_arch = "x86",
              target_arch = "arm",
              target_arch = "loongarch32",
              target_arch = "m68k",
