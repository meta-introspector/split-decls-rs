// MINIMAL TEST CASE for parsing failure in: ../rust/library/compiler-builtins/crates/symbol-check/src/main.rs
// Error: expected square brackets
// Problematic line: line 11

use std::process::{Command, Stdio};

use object::read::archive::{ArchiveFile, ArchiveMember};
use object::{
    File as ObjFile, Object, ObjectSection, ObjectSymbol, Symbol, SymbolKind, SymbolScope,
};
use serde_json::Value;
