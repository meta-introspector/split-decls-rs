// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/examples/hex.rs
// Error: expected square brackets
// Problematic line: line 18

#![allow(internal_features)]
#![feature(wasm_target_feature)]
#![cfg_attr(test, feature(test))]
#![cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature(stdarch_internal)
)]
