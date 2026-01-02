// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_builtin_macros/src/cfg_accessible.rs
// Error: expected square brackets
// Problematic line: line 13


pub(crate) struct Expander;

fn validate_input<'a>(ecx: &ExtCtxt<'_>, mi: &'a ast::MetaItem) -> Option<&'a ast::Path> {
    use errors::CfgAccessibleInvalid::*;
    match mi.meta_item_list() {
        None => {}
