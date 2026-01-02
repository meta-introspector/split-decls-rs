// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_session/src/filesearch.rs
// Error: expected square brackets
// Problematic line: line 11


use crate::search_paths::{PathKind, SearchPath};

#[derive(Clone)]
pub struct FileSearch {
    cli_search_paths: Vec<SearchPath>,
    tlib_path: SearchPath,
