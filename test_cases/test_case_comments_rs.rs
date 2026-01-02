// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/abi/comments.rs
// Error: expected square brackets
// Problematic line: line 10


use crate::prelude::*;

pub(super) fn add_args_header_comment(fx: &mut FunctionCx<'_, '_, '_>) {
    if fx.clif_comments.enabled() {
        fx.add_global_comment(
            "kind  loc.idx   param    pass mode                            ty".to_string(),
