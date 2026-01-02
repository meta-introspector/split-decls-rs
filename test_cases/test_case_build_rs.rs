// MINIMAL TEST CASE for parsing failure in: ../rust/library/profiler_builtins/build.rs
// Error: expected square brackets
// Problematic line: line 10

use std::env;
use std::path::PathBuf;

fn main() {
    if let Ok(rt) = tracked_env_var("LLVM_PROFILER_RT_LIB") {
        let rt = PathBuf::from(rt);
        if let Some(lib) = rt.file_name() {
