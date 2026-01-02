// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_builtin_macros/src/cmdline_attrs.rs
// Error: expected square brackets
// Problematic line: line 10

use rustc_session::parse::ParseSess;
use rustc_span::FileName;

pub fn inject(krate: &mut ast::Crate, psess: &ParseSess, attrs: &[String]) {
    for raw_attr in attrs {
        let source = format!("#![{raw_attr}]");
        let parse = || -> Result<ast::Attribute, Vec<Diag<'_>>> {
