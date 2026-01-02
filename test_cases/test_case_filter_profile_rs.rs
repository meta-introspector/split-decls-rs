// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/scripts/filter_profile.rs
// Error: expected square brackets
// Problematic line: line 2

#!/usr/bin/env bash
#![rustfmt::skip]/* This line is ignored by bash
# This block is ignored by rustc
pushd $(dirname "$0")/../
RUSTC="$(pwd)/dist/rustc-clif"
