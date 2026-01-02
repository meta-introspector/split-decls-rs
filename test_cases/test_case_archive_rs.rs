// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_llvm/src/back/archive.rs
// Error: expected square brackets
// Problematic line: line 6

use std::ffi::{CStr, c_char, c_void};
use std::io;

use rustc_codegen_ssa::back::archive::{
    ArArchiveBuilder, ArchiveBuilder, ArchiveBuilderBuilder, DEFAULT_OBJECT_READER, ObjectReader,
};
use rustc_session::Session;
