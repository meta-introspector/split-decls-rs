// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_builtin_macros/src/source_util.rs
// Error: expected square brackets
// Problematic line: line 11

use rustc_ast::tokenstream::TokenStream;
use rustc_ast::{join_path_idents, token};
use rustc_ast_pretty::pprust;
use rustc_expand::base::{
    DummyResult, ExpandResult, ExtCtxt, MacEager, MacResult, MacroExpanderResult, resolve_path,
};
use rustc_expand::module::DirOwnership;
