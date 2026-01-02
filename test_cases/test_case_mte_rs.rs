// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/aarch64/mte.rs
// Error: expected square brackets
// Problematic line: line 5

//!
//! [ACLE documentation](https://arm-software.github.io/acle/main/acle.html#markdown-toc-mte-intrinsics)

unsafe extern "unadjusted" {
    #[cfg_attr(
        any(target_arch = "aarch64", target_arch = "arm64ec"),
        link_name = "llvm.aarch64.irg"
