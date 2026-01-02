// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/alloc/xous.rs
// Error: expected square brackets
// Problematic line: line 6


use crate::alloc::{GlobalAlloc, Layout, System};

#[cfg(not(test))]
#[unsafe(export_name = "_ZN16__rust_internals3std3sys4xous5alloc8DLMALLOCE")]
static mut DLMALLOC: dlmalloc::Dlmalloc = dlmalloc::Dlmalloc::new();

